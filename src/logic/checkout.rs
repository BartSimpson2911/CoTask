use crate::storage::{
    head::{read_head, write_head},
    commit::load_commit,
};

pub fn checkout(commit_number: usize) {
    let current_head = match read_head() {
        Ok(h) => h,
        Err(_) => {
            println!("Repository not initialized.");
            return;
        }
    };

    if commit_number == 0 {
        println!("Invalid commit number.");
        return;
    }

    // Load commit once
    let _commit = match load_commit(commit_number) {
        Ok(c) => c,
        Err(_) => {
            println!("Commit {} does not exist.", commit_number);
            return;
        }
    };

    if let Err(_) = write_head(commit_number) {
        println!("Failed to update HEAD.");
        return;
    }

    println!("Switched from commit {} to commit {}", current_head,commit_number);
}
