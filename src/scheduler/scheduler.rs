use crate::structs::task::PeriodicTask;
use crate::structs::job::Job;

pub trait Scheduler {
    fn name(&self) -> &str {
        "UnnamedScheduler"
    }
    fn schedulability_test(&self, tasks: &Vec<PeriodicTask>) -> bool;
    fn sort_ready_queue(
        &self,
        queue: &mut Vec<Job>,
        tasks: &Vec<PeriodicTask>,
        current_time: usize
    );
}
