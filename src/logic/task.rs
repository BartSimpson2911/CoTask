use core::task;
use serde::{Deserialize, Serialize};
use crate::{models::task_model, storage::{commit::{self, load_commit, save_commit}, head::{self, read_head, write_head}}};
use crate::models::commit_model::Commit;
use crate::models::task_model::Task;




pub fn add_task(text: &str) {
    let head = match read_head() {
        Ok(h) => h,
        Err(_) => {
            println!("Repository is not initialized");
            return;
        }
    };

    let mut tasks = if head == 0 {
        Vec::new()
    } else {
        match load_commit(head) {
            Ok(commit) => commit.tasks,
            Err(_) => {
                println!("Failed to load previous commit");
                return;
            }
        }
    };

    // Create new task
    let new_id = tasks.len() + 1;

    let new_task = task_model::Task {
        id: new_id,
        text: text.to_string(),
        completed: false,
    };

    tasks.push(new_task);

    let head = read_head().expect("Failed to read head");
    let parent = if head == 0 { None } else { Some(head) };

    // Create new commit snapshot
    let new_commit = Commit {
    parents: if head == 0 { vec![] } else { vec![head] },
    tasks,
    };


    let new_commit_number = head + 1;

    if let Err(_) = save_commit(new_commit_number, &new_commit) {
        println!("Failed to save commit.");
        return;
    }

    if let Err(_) = write_head(new_commit_number) {
        println!("Failed to update HEAD.");
        return;
    }

    println!("Task added in commit {}", new_commit_number);
}


pub fn mark_done(id: usize) {
    let head = match  read_head() {
        Ok(h)=> h,
        Err(_) => {
            println!("Repository not initialized.");
            return;
        }
    };
    if (head ==0) {
        println!("No task were found!");
        return;
    }

    let mut commit = match load_commit(head) {
        Ok(c) => c,
        Err(_) => {
            println!("Failed to load commit.");
            return;
        }
    };

    // Find task
    let mut found=false;

    if let Some(task) = commit.tasks.iter_mut().find(|t| t.id == id) {
        if task.completed {
            println!("Task {} is already completed.", id);
            return;
        }

            task.completed = true;
    } else {
            println!("Task with id {} not found.", id);
            return;
        }

    if(!found){
        println!("Task with id {} not found.", id);
        return;
    }

    let new_commit_number=head+1;
    if let Err(_) = save_commit(new_commit_number, &commit) {
        println!("Failed to save commit.");
        return;
    }

    if let Err(_) = write_head(new_commit_number) {
        println!("Failed to update HEAD.");
        return;
    }

    println!("Task {} marked as done in commit {}", id, new_commit_number);

}