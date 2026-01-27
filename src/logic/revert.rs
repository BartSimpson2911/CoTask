use crate::storage::{
    head::{read_head, write_head},
    commit::{load_commit, save_commit},
};

pub fn revert(commit_number: usize) {
    let current_head = match read_head(){
        Ok(h) => h,
        Err(_) => {
            println!("Repository not initialized.");
            return;
        }

    };

    if commit_number == 0{
        println!("Invalid Commit number!");
        return;
    }

    // Load old commit state
    let old_commit = match load_commit(commit_number) {
        Ok(c) => c,
        Err(_) => {
            println!("Commit {} does not exist.", commit_number);
            return;
        }
    };

    // Create new snapshot from old state
    let new_commit_number = current_head + 1;

    if let Err(_) = save_commit(new_commit_number, &old_commit) {
        println!("Failed to save revert commit.");
        return;
    }

    if let Err(_) = write_head(new_commit_number) {
        println!("Failed to update HEAD.");
        return;
    }

    println!(
        "Reverted to commit {}. New commit {} created.",
        commit_number, new_commit_number
    );
}