use std::collections::HashSet;
use std::fs;

use crate::storage::{
    head::read_head,
    commit::load_commit,
};

fn collect_reachable(commit_number: usize, reachable: &mut HashSet<usize>) {
    // Avoid revisiting nodes
    if reachable.contains(&commit_number) {
        return;
    }

    // Try loading commit
    if let Ok(commit) = load_commit(commit_number) {
        reachable.insert(commit_number);

        // Traverse ALL parents (DAG traversal)
        for parent in commit.parents {
            collect_reachable(parent, reachable);
        }
    }
}
pub fn run_gc() {
    let head = match read_head() {
        Ok(h) => h,
        Err(_) => {
            println!("Repository not initialized.");
            return;
        }
    };

    if head == 0 {
        println!("No commits to clean.");
        return;
    }

    // Step 1: Find reachable commits
    let mut reachable = HashSet::new();
    collect_reachable(head, &mut reachable);

    // Step 2: Read all commit files
    let paths = match fs::read_dir(".cotask/commits") {
        Ok(p) => p,
        Err(_) => {
            println!("Failed to read commits directory.");
            return;
        }
    };

    let mut deleted = 0;

    for entry in paths.flatten() {
        if let Some(name) = entry.path().file_stem() {
            if let Ok(num) = name.to_string_lossy().parse::<usize>() {
                if !reachable.contains(&num) {
                    if fs::remove_file(entry.path()).is_ok() {
                        deleted += 1;
                    }
                }
            }
        }
    }

    println!("GC complete. Removed {} unreachable commits.", deleted);
}
