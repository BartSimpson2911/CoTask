use std::fs;
use crate::storage::{
    commit::load_commit,
    head::{read_head_branch, read_branch_commit, write_branch_commit},
    commit::save_commit,
};
use crate::models::commit_model::Commit;

pub fn stash() {
    let branch = read_head_branch().unwrap();
    let head = read_branch_commit(&branch).unwrap();
    let commit = load_commit(head).unwrap();

    let stash_id = next_stash_id();
    let path = format!(".cotask/stash/{}.json", stash_id);

    let data = serde_json::to_string_pretty(&commit).unwrap();
    fs::write(&path, data).unwrap();

    println!("Saved working state to stash {}", stash_id);
}

pub fn stash_pop() {
    let stash_id = latest_stash_id();

    if stash_id == 0 {
        println!("No stash entries.");
        return;
    }

    let path = format!(".cotask/stash/{}.json", stash_id);
    let data = fs::read_to_string(&path).unwrap();
    let commit: Commit = serde_json::from_str(&data).unwrap();

    let branch = read_head_branch().unwrap();
    let head = read_branch_commit(&branch).unwrap();

    let new_commit_number = head + 1;
    save_commit(new_commit_number, &commit).unwrap();
    write_branch_commit(&branch, new_commit_number).unwrap();

    fs::remove_file(path).unwrap();

    println!("Applied stash {} → new commit {}", stash_id, new_commit_number);
}

fn next_stash_id() -> usize {
    let mut max = 0;
    if let Ok(entries) = fs::read_dir(".cotask/stash") {
        for e in entries.flatten() {
            if let Some(name) = e.file_name().to_str() && let Ok(n) = name.trim_end_matches(".json").parse::<usize>() &&  n > max {        
                max = n;
            }
        }
    }
    max + 1
}

fn latest_stash_id() -> usize {
    let mut max = 0;
    if let Ok(entries) = fs::read_dir(".cotask/stash") {
        for e in entries.flatten() {
            if let Some(name) = e.file_name().to_str() && let Ok(n) = name.trim_end_matches(".json").parse::<usize>()  &&  n > max {
                max = n;
            }
        }
    }
    max
}
