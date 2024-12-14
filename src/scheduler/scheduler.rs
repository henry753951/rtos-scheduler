use crate::structs::task::Task;
use crate::structs::job::Job;

pub trait Scheduler {
    fn name(&self) -> &str {
        "UnnamedScheduler"
    }
    fn schedulability_test(&self, tasks: &Vec<Task>) -> bool;
    fn sort_ready_queue(&self, queue: &mut Vec<Job>, tasks: &Vec<Task>, current_time: usize);
}
