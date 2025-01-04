use crate::scheduler::scheduler::Scheduler;
use crate::structs::task::{PeriodicTask, TaskTrait};
use crate::structs::job::Job;

pub struct LSTScheduler;

impl Scheduler for LSTScheduler {
    fn name(&self) -> &str {
        "LSTScheduler"
    }

    fn schedulability_test(&self, _tasks: &Vec<PeriodicTask>) -> bool {
        true // Strict LST 不做 Schedulability Test
    }

    fn sort_ready_queue(&self, queue: &mut Vec<Job>, _tasks: &Vec<PeriodicTask>, current_time: usize) {
        // 依 Slack 時間從小到大排序，Slack 相同則依 task_id 排序
        queue.sort_by(|a, b| {
            let slack_a =
                (a.absolute_deadline as isize) -
                (current_time as isize) -
                (a.remaining_execution_time as isize);
            let slack_b =
                (b.absolute_deadline as isize) -
                (current_time as isize) -
                (b.remaining_execution_time as isize);
            if slack_a != slack_b {
                slack_a.cmp(&slack_b)
            } else {
                a.task_id.cmp(&b.task_id)
            }
        });
    }
}
