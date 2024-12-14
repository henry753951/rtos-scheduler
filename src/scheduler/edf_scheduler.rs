use crate::scheduler::scheduler::Scheduler;
use crate::structs::task::Task;
use crate::structs::job::Job;

pub struct EDFScheduler;

impl Scheduler for EDFScheduler {
    fn name(&self) -> &str {
        "EDFScheduler"
    }

    fn schedulability_test(&self, tasks: &Vec<Task>) -> bool {
        let utilization_sum: f64 = tasks
            .iter()
            .map(|task| task.utilization)
            .sum();

        utilization_sum <= 1f64;
        true
    }

    fn sort_ready_queue(&self, queue: &mut Vec<Job>, _tasks: &Vec<Task>, _current_time: usize) {
        // 依絕對截止時間從小到大排序，截止時間相同則依 task_id 排序
        queue.sort_by(|a, b| {
            if a.absolute_deadline != b.absolute_deadline {
                a.absolute_deadline.cmp(&b.absolute_deadline)
            } else {
                a.task_id.cmp(&b.task_id)
            }
        });
    }
}
