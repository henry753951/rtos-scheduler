use std::error::Error;
use std::path::Path;
use std::fs::{ self, create_dir_all, File };
use std::io::{ BufReader, BufRead, Write };
use std::collections::HashSet;
use plotters::prelude::*;
use crate::structs::task::{ AperiodicTask, PeriodicTask, TaskTrait };
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
pub fn max_phase(tasks: &Vec<Box<dyn TaskTrait>>) -> usize {
    tasks
        .iter()
        .filter(|task| !task.is_aperiodic()) // 只保留週期性任務
        .map(|task| {
            let periodic_task = task.as_any().downcast_ref::<PeriodicTask>().unwrap();
            periodic_task.phase
        })
        .max()
        .unwrap_or(0)
}
/// 計算所有任務週期的最小公倍數 (LCM)
pub fn lcm_of_periods(tasks: &Vec<Box<dyn TaskTrait>>) -> usize {
    tasks
        .iter()
        .filter(|task| !task.is_aperiodic()) // 只保留週期性任務
        .map(|task| {
            let periodic_task = task.as_any().downcast_ref::<PeriodicTask>().unwrap();
            periodic_task.period
        })
        .fold(1, |acc, period| lcm(acc, period as usize))
}

pub fn read_task_files(tasks_dir: &str) -> Vec<String> {
    fs::read_dir(tasks_dir)
        .expect("無法讀取目錄")
        .filter_map(|entry| {
            let entry = entry.expect("無法讀取檔案");
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "txt" {
                        let mut path_str = path.to_str().unwrap().to_string();
                        let ext_len = ext.to_str().unwrap().len() + 1; // 加上 '.' 的長度
                        path_str.truncate(path_str.len() - ext_len);
                        return Some(path_str);
                    }
                }
            }
            None
        })
        .collect::<Vec<String>>()
}

/// 讀取Task檔案
pub fn read_tasks(filename: &str) -> Vec<Box<dyn TaskTrait>> {
    let mut tasks = Vec::new();
    // 讀取週期性任務
    let periodic_tasks = read_tasks_periodic(&(filename.to_string() + ".txt"));
    tasks.extend(periodic_tasks.into_iter());
    // 讀取非週期性任務
    let aperiodic_tasks = read_tasks_aperiodic(&(filename.to_string() + ".aperiodic.txt"));
    tasks.extend(aperiodic_tasks.into_iter());
    tasks
}

pub fn get_periodic_tasks(tasks: Vec<Box<dyn TaskTrait>>) -> Vec<PeriodicTask> {
    tasks
        .iter()
        .filter(|task| !task.is_aperiodic()) // 只保留週期性任務
        .map(|task| {
            let periodic_task = task.as_any().downcast_ref::<PeriodicTask>().unwrap();
            (*periodic_task).clone()
        })
        .collect()
}

pub fn get_aperiodic_tasks(tasks: Vec<Box<dyn TaskTrait>>) -> Vec<AperiodicTask> {
    tasks
        .iter()
        .filter(|task| task.is_aperiodic()) // 只保留非週期性任務
        .map(|task| {
            let aperiodic_task = task.as_any().downcast_ref::<AperiodicTask>().unwrap();
            (*aperiodic_task).clone()
        })
        .collect()
}

pub fn read_tasks_periodic(filename: &str) -> Vec<Box<dyn TaskTrait>> {
    let mut tasks = Vec::new();

    let file = File::open(filename);
    if file.is_err() {
        eprintln!("無法打開{}任務檔案", filename);
        return tasks;
    }
    let file = file.unwrap();
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
        if parts.len() == 4 {
            let phase: usize = parts[0].parse().expect("無效的數字");
            let period: usize = parts[1].parse().expect("無效的數字");
            let relative_deadline: usize = parts[2].parse().expect("無效的數字");
            let worst_case_execution_time: usize = parts[3].parse().expect("無效的數字");

            let task: Box<dyn TaskTrait> = Box::new(PeriodicTask {
                task_id: line_num + 1,
                phase,
                period,
                worst_case_execution_time,
                relative_deadline: relative_deadline,
            });
            tasks.push(task);
        } else if parts.len() == 2 {
            let period: usize = parts[0].parse().expect("無效的數字");
            let worst_case_execution_time: usize = parts[1].parse().expect("無效的數字");

            let task: Box<dyn TaskTrait> = Box::new(PeriodicTask {
                task_id: line_num + 1,
                phase: 0,
                period,
                worst_case_execution_time,
                relative_deadline: worst_case_execution_time,
            });
            tasks.push(task);
        } else {
            eprintln!("第 {} 行格式錯誤", line_num + 1);
            continue;
        }
    }
    tasks
}

pub fn read_tasks_aperiodic(filename: &str) -> Vec<Box<dyn TaskTrait>> {
    let mut tasks = Vec::new();

    let file = File::open(filename);
    if file.is_err() {
        eprintln!("無法打開{}任務檔案", filename);
        return tasks;
    }
    let file = file.unwrap();
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
        if parts.len() == 2 {
            // 解析各個字段
            let arrival_time: usize = parts[0].parse().expect("無效的數字");
            let worst_case_execution_time: usize = parts[1].parse().expect("無效的數字");

            // 創建 AperiodicTask 並加入 tasks 向量
            let task: Box<dyn TaskTrait> = Box::new(AperiodicTask {
                task_id: line_num + 1,
                arrival_time,
                worst_case_execution_time,
                relative_deadline: worst_case_execution_time,
            });
            tasks.push(task);
        } else if parts.len() == 3 {
            // 解析各個字段
            let arrival_time: usize = parts[0].parse().expect("無效的數字");
            let worst_case_execution_time: usize = parts[1].parse().expect("無效的數字");
            let relative_deadline: usize = parts[2].parse().expect("無效的數字");

            // 創建 AperiodicTask 並加入 tasks 向量
            let task: Box<dyn TaskTrait> = Box::new(AperiodicTask {
                task_id: line_num + 1,
                arrival_time,
                worst_case_execution_time,
                relative_deadline: relative_deadline,
            });
            tasks.push(task);
        } else {
            eprintln!("第 {} 行格式錯誤", line_num + 1);
            continue;
        }
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
