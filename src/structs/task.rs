pub struct Task {
    pub task_id: usize, // 任務識別碼
    pub phase: usize, // 初始階段時間
    pub period: usize, // 週期
    pub worst_case_execution_time: usize, // 最壞情況執行時間
    pub relative_deadline: usize, // 相對截止時間
    pub utilization: f64, // 利用率
}
