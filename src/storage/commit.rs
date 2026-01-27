use std::fs;
use std::io;
use crate::models::commit_model::Commit;

pub fn save_commit(commit_number: usize, commit: &Commit) -> io::Result<()> {
    let path = format!(".cotask/commits/{}.json", commit_number);
    let data = serde_json::to_string_pretty(commit)?;
    fs::write(path, data)?;
    Ok(())
}

pub fn load_commit(commit_number: usize) -> io::Result<Commit> {
    let path = format!(".cotask/commits/{}.json", commit_number);
    let data = fs::read_to_string(path)?;
    let commit: Commit = serde_json::from_str(&data)?;
    Ok(commit)
}
