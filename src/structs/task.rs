use std::any::Any;

pub trait TaskTrait {
    fn task_id(&self) -> usize;
    fn worst_case_execution_time(&self) -> usize;
    fn relative_deadline(&self) -> usize;
    fn is_aperiodic(&self) -> bool;
    fn as_any(&self) -> &dyn Any;
}
#[derive(Clone)]
pub struct AperiodicTask {
    pub task_id: usize, // 任務識別碼
    pub arrival_time: usize, // 抵達時間
    pub worst_case_execution_time: usize, // 最壞情況執行時間
    pub relative_deadline: usize, // 相對截止時間
}

impl TaskTrait for AperiodicTask {
    fn task_id(&self) -> usize {
        self.task_id
    }

    fn worst_case_execution_time(&self) -> usize {
        self.worst_case_execution_time
    }

    fn relative_deadline(&self) -> usize {
        self.relative_deadline
    }

    fn is_aperiodic(&self) -> bool {
        true
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
#[derive(Clone)]
pub struct PeriodicTask {
    pub task_id: usize, // 任務識別碼
    pub phase: usize, // 初始階段時間
    pub period: usize, // 週期
    pub worst_case_execution_time: usize, // 最壞情況執行時間
    pub relative_deadline: usize, // 相對截止時間
}

impl TaskTrait for PeriodicTask {
    fn task_id(&self) -> usize {
        self.task_id
    }

    fn worst_case_execution_time(&self) -> usize {
        self.worst_case_execution_time
    }

    fn relative_deadline(&self) -> usize {
        self.relative_deadline
    }

    fn is_aperiodic(&self) -> bool {
        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
