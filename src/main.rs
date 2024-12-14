use std::path::Path;

mod utils;
mod structs;
mod scheduler;

use scheduler::scheduler::Scheduler;
use scheduler::rm_scheduler::RMScheduler;
use scheduler::edf_scheduler::EDFScheduler;
use scheduler::lst_scheduler::LSTScheduler;
use structs::{ job::Job, tick_info::TickInfo };
use utils::{ lcm_of_periods, max_phase, read_task_files, read_tasks, write_output };

fn main() {
    // 定義任務目錄
    let tasks_dir = "inputs/";

    // 確認任務目錄存在
    if !Path::new(tasks_dir).is_dir() {
        eprintln!("'{}' 不存在", tasks_dir);
        return;
    }

    let task_files = read_task_files(tasks_dir);
    if task_files.is_empty() {
        eprintln!("'{}' 中沒有 .txt 檔案", tasks_dir);
        return;
    }

    // 定義所有排程器
    let schedulers: Vec<Box<dyn Scheduler>> = vec![
        Box::new(RMScheduler),
        Box::new(EDFScheduler),
        Box::new(LSTScheduler)
    ];

    // 遍歷每個任務檔案
    for task_file in task_files {
        println!("👀 {}", task_file);

        // 讀取任務
        let tasks = read_tasks(&task_file);

        // 計算最小公倍數和最大相位
        let lcm_value = lcm_of_periods(&tasks);
        let max_ph = max_phase(&tasks);
        // 結束時間
        let end_time = lcm_value + max_ph;

        for scheduler in schedulers.iter() {
            let scheduler_name = scheduler.name();

            // SECTION [init]
            let mut ready_queue: Vec<Job> = Vec::new();
            let mut clock = 0;
            // count
            let mut total_job_number = 0;
            let mut miss_deadline_job_number = 0;

            // SECTION [test]
            if !scheduler.schedulability_test(&tasks) {
                println!("\n🔥{}: 任務不可排程", scheduler_name);
                let _ = write_output(scheduler_name, &task_file, &Vec::new());
                continue;
            } else {
                println!("\n🔥{}: 任務可排程", scheduler_name);
            }

            let mut output_data: Vec<TickInfo> = Vec::new();

            // SECTION [simulation]
            while clock < end_time {
                let mut arrivals: Vec<usize> = Vec::new();
                let mut deadlines: Vec<usize> = Vec::new();

                // SECTION [截止處理]
                // absolute_deadline - clock - remaining_execution_time < 0
                let mut to_remove_indices = Vec::new();
                for (i, job) in ready_queue.iter().enumerate() {
                    let remaining_time =
                        (job.absolute_deadline as isize) -
                        (clock as isize) -
                        (job.remaining_execution_time as isize);
                    if remaining_time < 0 {
                        miss_deadline_job_number += 1;
                        to_remove_indices.push(i);

                        println!("\tt: {}, 錯過截止時間 task: {}", clock, job.task_id);
                        deadlines.push(job.task_id.clone());
                    }
                }
                // 移除錯過截止時間的工作
                for &i in to_remove_indices.iter().rev() {
                    ready_queue.remove(i);
                }

                // SECTION [抵達處理]
                // phase <= clock && (clock - phase) % period == 0
                for task in &tasks {
                    if clock >= task.phase && (clock - task.phase) % task.period == 0 {
                        let job = Job {
                            release_time: clock,
                            remaining_execution_time: task.worst_case_execution_time,
                            absolute_deadline: clock + task.relative_deadline,
                            task_id: task.task_id,
                        };
                        ready_queue.push(job);
                        total_job_number += 1;

                        println!("\tt: {}, 抵達 task: {}", clock, task.task_id);
                        arrivals.push(task.task_id);
                    }
                }

                // SECTION [排序就緒佇列]
                scheduler.sort_ready_queue(&mut ready_queue, &tasks, clock);

                // SECTION [執行]
                // 執行最高優先權的工作
                let running = if let Some(executing_job) = ready_queue.first() {
                    println!("\tt: {}, 執行 task: {}", clock, executing_job.task_id);
                    Some(executing_job.task_id.clone())
                } else {
                    None
                };

                // 執行最高優先權的工作
                if let Some(executing_job) = ready_queue.first_mut() {
                    executing_job.remaining_execution_time -= 1;

                    if executing_job.remaining_execution_time == 0 {
                        // 完成工作，從佇列中移除
                        ready_queue.remove(0);
                    }
                }

                // 建立 TickInfo
                let tick_info = TickInfo {
                    t: clock,
                    running: running,
                    arrival: arrivals,
                    dead: deadlines,
                };
                output_data.push(tick_info);

                // SECTION [clock++]
                clock += 1;
            }

            // 輸出統計資訊
            let stats = format!(
                "\t總共工作數: {}\n\t錯過截止時間的工作數: {}\n",
                total_job_number,
                miss_deadline_job_number
            );
            println!("{}", stats.trim_end());

            // 將排程結果寫入檔案
            let _ = write_output(scheduler_name, &task_file, &output_data);
        }
        println!("----------------------------------------");
    }
}
