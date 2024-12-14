pub struct Job {
    pub release_time: usize, // 發布時間
    pub remaining_execution_time: usize, // 剩餘執行時間
    pub absolute_deadline: usize, // 絕對截止時間
    pub task_id: usize, // 屬於哪個 Task 的工作
}
