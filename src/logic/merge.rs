use std::fs;
use std::collections::HashMap;
use crate::storage::{
    head::{read_head_branch, read_branch_commit, write_branch_commit},
    commit::{load_commit, save_commit},
};
use crate::models::{commit_model::Commit, task_model::Task};

pub fn merge_branch(target_branch: &str) {
    // Current branch (OURS)
    let current_branch = match read_head_branch() {
        Ok(b) => b,
        Err(_) => {
            println!("Repository not initialized.");
            return;
        }
    };

    if current_branch == target_branch {
        println!("Cannot merge a branch into itself.");
        return;
    }

    // Commit numbers
    let ours = match read_branch_commit(&current_branch) {
        Ok(c) => c,
        Err(_) => {
            println!("Failed to read current branch commit.");
            return;
        }
    };

    let theirs = match read_branch_commit(target_branch) {
        Ok(c) => c,
        Err(_) => {
            println!("Target branch '{}' does not exist.", target_branch);
            return;
        }
    };

    // Load commits
    let current = load_commit(ours).unwrap();
    let target = load_commit(theirs).unwrap();

    let mut task_map: HashMap<usize, Task> = HashMap::new();
    let mut conflict_found = false;

    // Insert OUR tasks
    for t in current.tasks {
        task_map.insert(t.id, t);
    }

    // Merge THEIR tasks
    for t in target.tasks {
        task_map
            .entry(t.id)
            .and_modify(|existing| {
                // Conflict detection
                if existing.completed != t.completed || existing.text != t.text {
                    let conflict_info = format!(
                        "TASK {}\nOURS: completed={}, text={}\nTHEIRS: completed={}, text={}\n",
                        existing.id, existing.completed, existing.text,
                        t.completed, t.text
                    );
                    fs::write(".cotask/MERGE_CONFLICT", conflict_info).unwrap();

                    println!("⚠ Conflict in task {}!", existing.id);
                    println!("Run: cotask resolve {} done|undone", existing.id);

                    conflict_found = true;
                }
            })
            .or_insert(t);
    }

    //  Stop merge safely if conflict happened
    if conflict_found {
        return;
    }

    let merged_tasks: Vec<Task> = task_map.into_values().collect();

    let new_commit_number = ours + 1;
    let message = format!("Merged branch '{}'", target_branch);

    let new_commit = Commit {
        parents: vec![ours, theirs],
        message,
        tasks: merged_tasks,
    };

    save_commit(new_commit_number, &new_commit).unwrap();
    write_branch_commit(&current_branch, new_commit_number).unwrap();

    println!(
        "Merged branch '{}' into '{}' → commit {}",
        target_branch, current_branch, new_commit_number
    );
}
