use serde::{Deserialize, Serialize};
use crate::models::task_model::Task;
#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    pub parents: Vec<usize>,   // multiple parents
    pub tasks: Vec<Task>,
}
