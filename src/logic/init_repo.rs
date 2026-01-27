use std::{fs, path::Path};

pub fn init_repo() {
    let repo_path = Path::new(".cotask");

    if repo_path.exists() {
        println!("Repository already initialized.");
        return;
    }

    // Create base directory
    fs::create_dir(repo_path).expect("Failed to create .cotask directory");

    // Create commits folder
    fs::create_dir(repo_path.join("commits"))
        .expect("Failed to create commits directory");

    // Create refs folder
    fs::create_dir(repo_path.join("refs"))
        .expect("Failed to create refs directory");

    // HEAD now stores branch name
    fs::write(repo_path.join("HEAD"), "main")
        .expect("Failed to create HEAD");

    // Create main branch pointing to commit 0
    fs::write(repo_path.join("refs/main"), "0")
        .expect("Failed to create main branch");

    println!("Initialized empty cotask repository with branch 'main'.");
}
