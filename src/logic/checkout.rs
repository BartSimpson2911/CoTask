use std::fs;
use crate::storage::{
    commit::load_commit,
    head::{write_head_branch, read_head_branch},
};
use crate::logic::tag::read_tag_commit;

pub fn checkout_commit(commit_number: usize) {
    if commit_number == 0 {
        println!("Invalid commit number.");
        return;
    }

    if load_commit(commit_number).is_err() {
        println!("Commit {} does not exist.", commit_number);
        return;
    }

    if fs::write(".cotask/HEAD", commit_number.to_string()).is_err() {
        println!("Failed to detach HEAD.");
        return;
    }

    println!("HEAD detached at commit {}", commit_number);
}

pub fn checkout_ref(name: &str) {
    // 1) Branch check
    let branch_path = format!(".cotask/refs/{}", name);
    if fs::metadata(&branch_path).is_ok() {
        if let Ok(current) = read_head_branch() && current == name  {
            {
                println!("Already on branch '{}'", name);
                return;
            }
        }

        if write_head_branch(name).is_err() {
            println!("Failed to switch branch.");
            return;
        }

        println!("Switched to branch '{}'", name);
        return;
    }

    // 2) Tag check
    if let Some(commit) = read_tag_commit(name) {
        if fs::write(".cotask/HEAD", commit.to_string()).is_err() {
            println!("Failed to detach HEAD.");
            return;
        }

        println!("HEAD detached at tag '{}' (commit {})", name, commit);
        return;
    }

    println!("Branch or tag '{}' not found.", name);
}
