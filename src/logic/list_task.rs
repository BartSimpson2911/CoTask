use crate::storage::{
    head::read_head,
    commit::load_commit,
};

pub fn list_tasks() {
    let head = match read_head() {
        Ok(h) => h,
        Err(_) => {
            println!("Repository not initialized.");
            return;
        }
    };

    if head == 0 {
        println!("No tasks yet.");
        return;
    }

    let commit = match load_commit(head) {
        Ok(c) => c,
        Err(_) => {
            println!("Failed to load latest commit.");
            return;
        }
    };

    println!("Tasks:\n");

    for task in commit.tasks {
        let status = if task.completed { "✓" } else { " " };
        println!("[{}] {}. {}", status, task.id, task.text);
    }
}
