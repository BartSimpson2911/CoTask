use std::collections::HashMap;
use crate::storage::{
    head::{read_head, write_head},
    commit::{load_commit, save_commit},
};
use crate::models::commit_model::Commit;
use crate::models::task_model::Task;

pub fn merge(target_commit: usize) {
    // 1️⃣ Read current HEAD
    let head = match read_head() {
        Ok(h) => h,
        Err(_) => {
            println!("Repository not initialized.");
            return;
        }
    };

    // 2️⃣ Load current commit (ours)
    let current = match load_commit(head) {
        Ok(c) => c,
        Err(_) => {
            println!("Failed to load current commit.");
            return;
        }
    };

    // 3️⃣ Load target commit (theirs)
    let target = match load_commit(target_commit) {
        Ok(c) => c,
        Err(_) => {
            println!("Target commit does not exist.");
            return;
        }
    };

    // 4️⃣ Merge tasks by ID
    let mut task_map: HashMap<usize, Task> = HashMap::new();

    // Insert current tasks
    for task in current.tasks {
        task_map.insert(task.id, task);
    }

    // Merge target tasks
    for task in target.tasks {
        task_map
            .entry(task.id)
            .and_modify(|t| {
                // Merge rule: if either completed → completed
                t.completed = t.completed || task.completed;
            })
            .or_insert(task);
    }

    let merged_tasks: Vec<Task> = task_map.into_values().collect();

    // 5️⃣ Create new commit with parent link
    let new_commit_number = head + 1;

    let new_commit = Commit {
        parents: vec![head,target_commit],
        tasks: merged_tasks,
    };

    // 6️⃣ Save new commit
    if save_commit(new_commit_number, &new_commit).is_err() {
        println!("Failed to save merged commit.");
        return;
    }

    // 7️⃣ Update HEAD
    if write_head(new_commit_number).is_err() {
        println!("Failed to update HEAD.");
        return;
    }

    println!(
        "Merged commit {} into new commit {}",
        target_commit, new_commit_number
    );
}
