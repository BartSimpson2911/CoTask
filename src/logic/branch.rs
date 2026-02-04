use std::fs;
use crate::storage::{head::{read_branch_commit, read_head_branch}};

pub fn create_branch(name: &str) {
    let current_branch = match read_head_branch() {
        Ok(b) => b,
        Err(_) => {
            println!("Repository not initialized.");
            return;
        }
    };

    let commit = match read_branch_commit(&current_branch) {
        Ok(c) => c,
        Err(_) => {
            println!("Failed to read current branch commit.");
            return;
        }
    };

    let path = format!(".cotask/refs/{}", name);

    if fs::metadata(&path).is_ok() {
        println!("Branch '{}' already exists.", name);
        return;
    }

    if fs::write(&path, commit.to_string()).is_err() {
        println!("Failed to create branch.");
        return;
    }

    println!("Branch '{}' created at commit {}", name, commit);
}

pub fn list_branches() {
    let head = read_head_branch().unwrap_or_default();

    let entries = match fs::read_dir(".cotask/refs") {
        Ok(e) => e,
        Err(_) => {
            println!("No branches found.");
            return;
        }
    };

    for entry in entries.flatten() {
        if let Some(name) = entry.file_name().to_str() {
            if name == head {
                println!("* {}", name); // current branch
            } else {
                println!("  {}", name);
            }
        }
    }
}

pub fn delete_branch(name: &str) {
    let current_branch = match read_head_branch() {
        Ok(b) => b,
        Err(_) => {
            println!("Repository not initialized.");
            return;
        }

    };
    // Prevent deleting current branch
    if name == current_branch {
        println!("Cannot delete the current branch '{}'.", name);
        return;
    }

    let path = format!(".cotask/refs/{}",name);
    // Check branch exists
    if fs::metadata(&path).is_err() {
        println!("Branch '{}' does not exist.", name);
        return;
    }

    // Delete branch ref
    if fs::remove_file(&path).is_err() {
        println!("Failed to delete branch '{}'.", name);
        return;
    }

    println!("Branch '{}' deleted.",name);
}