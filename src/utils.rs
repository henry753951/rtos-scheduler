use std::error::Error;
use std::path::Path;
use std::fs::{ self, create_dir_all, File };
use std::io::{ BufReader, BufRead, Write };
use std::cmp::min;
use std::collections::HashSet;
use plotters::prelude::*;
use crate::structs::task::Task;
use crate::structs::tick_info::TickInfo;

/// 計算最大公因數 (GCD)
pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// 計算最小公倍數 (LCM)
pub fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

/// 計算所有任務的最大 Phase 時間
pub fn max_phase(tasks: &Vec<Task>) -> usize {
    tasks
        .iter()
        .map(|task| task.phase)
        .max()
        .unwrap_or(0)
}

/// 計算所有任務週期的最小公倍數 (LCM)
pub fn lcm_of_periods(tasks: &Vec<Task>) -> usize {
    tasks.iter().fold(1, |acc, task| lcm(acc, task.period))
}

/// 讀取任務檔案名稱
pub fn read_task_files(tasks_dir: &str) -> Vec<String> {
    fs::read_dir(tasks_dir)
        .expect("無法讀取目錄")
        .filter_map(|entry| {
            let entry = entry.expect("無法讀取檔案");
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "txt" {
                        return Some(path.to_str().unwrap().to_string());
                    }
                }
            }
            None
        })
        .collect::<Vec<String>>()
}

/// 讀取Task檔案
pub fn read_tasks(filename: &str) -> Vec<Task> {
    let mut tasks = Vec::new();

    let file = File::open(filename).expect("無法打開任務檔案");
    let reader = BufReader::new(file);

    for (line_num, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line
            .split(',')
            .map(|s| s.trim())
            .collect();
        if parts.len() != 4 {
            eprintln!("第 {} 行格式錯誤", line_num + 1);
            continue;
        }
        let phase: usize = parts[0].parse().expect("無效的數字");
        let period: usize = parts[1].parse().expect("無效的數字");
        let relative_deadline: usize = parts[2].parse().expect("無效的數字");
        let worst_case_execution_time: usize = parts[3].parse().expect("無效的數字");
        let utilization =
            (worst_case_execution_time as f64) / (min(period, relative_deadline) as f64);
        let task = Task {
            task_id: line_num + 1,
            phase,
            period,
            worst_case_execution_time,
            relative_deadline,
            utilization,
        };
        tasks.push(task);
    }

    tasks
}

/// 寫入輸出檔案
pub fn write_output(
    scheduler_name: &str,
    task_file: &str,
    data: &Vec<TickInfo>
) -> Result<(), Box<dyn Error>> {
    let task_file_name = Path::new(task_file)
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .replace(".txt", "");
    let path = format!("outputs/{}/", scheduler_name);
    std::fs::create_dir_all(&path)?;

    //JSON
    let json_output = serde_json::to_string_pretty(&data)?;
    let json_file_path = format!("{}{}.json", path, task_file_name);
    let mut json_file = File::create(&json_file_path)?;
    json_file.write_all(json_output.as_bytes())?;

    // TXT
    let txt_file_path = format!("{}{}.txt", path, task_file_name);
    let mut txt_file = File::create(&txt_file_path)?;
    for tick in data {
        let line = if let Some(running_task) = tick.running {
            format!("{} T{}\n", tick.t, running_task)
        } else {
            format!("{} free\n", tick.t)
        };
        txt_file.write_all(line.as_bytes())?;
    }
    if data.is_empty() {
        let line = "無法排程任務\n".to_string();
        txt_file.write_all(line.as_bytes())?;
    }

    // PNG
    generate_gantt_chart(scheduler_name, &task_file_name, data)?;
    Ok(())
}

pub fn generate_gantt_chart(
    scheduler_name: &str,
    filename: &str,
    data: &Vec<TickInfo>
) -> Result<(), Box<dyn Error>> {
    if data.is_empty() {
        return Ok(());
    }

    let output_path = format!("outputs/{}/", scheduler_name);
    create_dir_all(&output_path)?;
    let png_file_path = format!("{}{}.png", output_path, filename);

    // 取得所有任務 ID，並排序
    let mut tasks_in_schedule = HashSet::new();
    for tick in data {
        if let Some(running_task) = tick.running {
            tasks_in_schedule.insert(running_task);
        }
        for &task in &tick.arrival {
            tasks_in_schedule.insert(task);
        }
        for &task in &tick.dead {
            tasks_in_schedule.insert(task);
        }
    }
    let mut tasks: Vec<usize> = tasks_in_schedule.into_iter().collect();
    tasks.sort();

    // 創建任務 ID 到 Y 位置的映射
    let task_positions: std::collections::HashMap<usize, usize> = tasks
        .iter()
        .enumerate()
        .map(|(i, &task)| (task, i))
        .collect();

    let unit = 10;
    let task_height = tasks.len() * unit;
    let time_span = data.len() * unit;

    // 定義圖表寬高
    let width = 1600;
    let height = (80 + 60 * tasks.len()) as u32;
    let root_area = BitMapBackend::new(&png_file_path, (width, height)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .margin(20)
        .x_label_area_size(80)
        .y_label_area_size(120)
        .build_cartesian_2d(0usize..time_span, 0usize..task_height)?;

    // 配置圖表
    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc("Time")
        .y_desc("Tasks")
        .y_labels(tasks.len())
        .y_label_formatter(
            &(|y| {
                let fix_y = *y / 10;
                if fix_y < tasks.len() {
                    format!("Task {}", tasks[fix_y])
                } else {
                    String::new()
                }
            })
        )
        .x_label_formatter(&(|x| format!("{}", x / 10)))
        .label_style(("sans-serif", 20))
        .axis_desc_style(("sans-serif", 24))
        .draw()?;

    // 畫 RunningTask
    let gray = RGBColor(128, 128, 128);
    for tick in data {
        if let Some(task_id) = tick.running {
            if let Some(&y) = task_positions.get(&task_id) {
                chart.draw_series(
                    std::iter::once(
                        Rectangle::new(
                            [
                                (tick.t * unit, y * unit),
                                (tick.t * unit + unit, y * unit + unit),
                            ],
                            gray.filled()
                        )
                    )
                )?;
            }
        }
    }

    // 標示 Arrival 時間點
    for tick in data {
        for &task_id in &tick.arrival {
            if let Some(&y) = task_positions.get(&task_id) {
                chart.draw_series(
                    std::iter::once(
                        Rectangle::new(
                            [
                                (tick.t * unit, y * unit + 2),
                                (tick.t * unit + 1, y * unit + unit - 2),
                            ],
                            GREEN.filled()
                        )
                    )
                )?;
            }
        }
    }

    // 標示 DeadlineMissed 時間點
    for tick in data {
        for &task_id in &tick.dead {
            if let Some(&y) = task_positions.get(&task_id) {
                chart.draw_series(
                    std::iter::once(
                        Rectangle::new(
                            [
                                (tick.t * unit, y * unit + 4),
                                (tick.t * unit + 1, y * unit + unit - 4),
                            ],
                            RED.filled()
                        )
                    )
                )?;
            }
        }
    }

    Ok(())
}
