use std::{collections::HashMap, fs};
use crate::{
    models::export_model::RepoExport,
    storage::commit::load_commit,
};

pub fn export_repo() {
    let mut commits = HashMap::new();
    let mut branches = HashMap::new();
    let mut tags = HashMap::new();

    // 📌 Load commits
    if let Ok(entries) = fs::read_dir(".cotask/commits") {
        for e in entries.flatten() {
            let name = e.file_name().into_string().unwrap();
            if let Ok(num) = name.trim_end_matches(".json").parse::<usize>() && let Ok(commit) = load_commit(num)  {
                
                commits.insert(num, commit);
                
            }
        }
    }

    // 🌿 Branches
    if let Ok(entries) = fs::read_dir(".cotask/refs") {
        for e in entries.flatten() {
            let name = e.file_name().into_string().unwrap();
            let path = format!(".cotask/refs/{}", name);
            let commit = fs::read_to_string(path).unwrap().trim().parse().unwrap();
            branches.insert(name, commit);
        }
    }

    // 🏷 Tags
    if let Ok(entries) = fs::read_dir(".cotask/tags") {
        for e in entries.flatten() {
            let name = e.file_name().into_string().unwrap();
            let path = format!(".cotask/tags/{}", name);
            let commit = fs::read_to_string(path).unwrap().trim().parse().unwrap();
            tags.insert(name, commit);
        }
    }

    let head = fs::read_to_string(".cotask/HEAD").unwrap();

    let export = RepoExport { commits, branches, tags, head };

    let json = serde_json::to_string_pretty(&export).unwrap();
    fs::write("cotask_backup.json", json).unwrap();

    println!("Repository exported to cotask_backup.json");
}
