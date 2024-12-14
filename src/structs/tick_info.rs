use serde::Serialize;

#[derive(Serialize)]
pub struct TickInfo {
    pub t: usize,
    pub running: Option<usize>,
    pub arrival: Vec<usize>,
    pub dead: Vec<usize>,
}
