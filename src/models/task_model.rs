use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Task {
    pub id: usize,
    pub text: String,
    pub completed: bool
}