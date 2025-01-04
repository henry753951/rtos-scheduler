use crate::scheduler::scheduler::Scheduler;
use crate::structs::task::{ self, PeriodicTask, TaskTrait };
use crate::structs::job::Job;

pub struct RMScheduler;

impl Scheduler for RMScheduler {
    fn name(&self) -> &str {
        "RMScheduler"
    }

    fn schedulability_test(&self, tasks: &Vec<PeriodicTask>) -> bool {
        let n = tasks.len() as f64;
        let utilization_sum: f64 = tasks
            .iter()
            .map(|task| {
                let execution_time = task.worst_case_execution_time as f64;
                let min_period_deadline = std::cmp::min(task.period, task.relative_deadline) as f64;
                let utilization = execution_time / min_period_deadline;
                utilization
            })
            .sum();
        let bound = n * ((2f64).powf(1f64 / n) - 1f64);
        utilization_sum <= bound;
        true
    }

    fn sort_ready_queue(
        &self,
        queue: &mut Vec<Job>,
        tasks: &Vec<PeriodicTask>,
        _current_time: usize
    ) {
        // 依週期從小到大排序，週期相同則依 task_id 排序
        queue.sort_by(|a, b| {
            let task_a = tasks[a.task_id - 1]
                .as_any()
                .downcast_ref::<task::PeriodicTask>()
                .unwrap();
            let task_b = tasks[b.task_id - 1]
                .as_any()
                .downcast_ref::<task::PeriodicTask>()
                .unwrap();
            if task_a.period != task_b.period {
                task_a.period.cmp(&task_b.period)
            } else {
                a.task_id.cmp(&b.task_id)
            }
        });
    }
}
