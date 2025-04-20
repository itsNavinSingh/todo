use bincode::{Decode, Encode};
#[derive(Encode, Decode, Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub priority: String,
    pub completed: bool,
    pub created_at: String,
    pub due: String,
}

#[derive(Encode, Decode, Debug)]
pub struct TaskList {
    pub tasks: Vec<Task>,
    pub next_id: u32,
}