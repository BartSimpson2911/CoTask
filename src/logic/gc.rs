use std::collections::HashSet;
use std::fs;

use crate::storage::{
    head::{read_head_branch, read_branch_commit},
    commit::load_commit,
};

fn collect_reachable(commit_number: usize, reachable: &mut HashSet<usize>) {
    // Avoid revisiting nodes
    if reachable.contains(&commit_number) {
        return;
    }

    // Load commit
    if let Ok(commit) = load_commit(commit_number) {
        reachable.insert(commit_number);

        // Traverse ALL parents (DAG traversal)
        for parent in commit.parents {
            collect_reachable(parent, reachable);
        }
    }
}

pub fn run_gc() {
    // 1️⃣ Read current branch
    let branch = match read_head_branch() {
        Ok(b) => b,
        Err(_) => {
            println!("Repository not initialized.");
            return;
        }
    };

    // 2️⃣ Read commit pointed by branch
    let head_commit = match read_branch_commit(&branch) {
        Ok(c) => c,
        Err(_) => {
            println!("Branch commit not found.");
            return;
        }
    };

    // 3️⃣ Mark reachable commits
    let mut reachable = HashSet::new();
    collect_reachable(head_commit, &mut reachable);

    // 4️⃣ Sweep unreachable commits
    let paths = match fs::read_dir(".cotask/commits") {
        Ok(p) => p,
        Err(_) => {
            println!("Failed to read commits directory.");
            return;
        }
    };

    let mut deleted = 0;

    for entry in paths.flatten() {
        if let Some(name) = entry.path().file_stem()
            && let Ok(num) = name.to_string_lossy().parse::<usize>()
            && !reachable.contains(&num)
            && fs::remove_file(entry.path()).is_ok() {
                deleted += 1;
            }

        }

    println!("GC complete. Removed {} unreachable commits.", deleted);
}
