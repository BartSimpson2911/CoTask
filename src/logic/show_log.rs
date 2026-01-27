use crate::storage::commit::load_commit;
use crate::storage::head::read_head;

pub fn show_log() {
    let head = match read_head() {
        Ok(h) => h,
        Err(_) => {
            println!("Repository not initialized.");
            return;
        }
    };

    if head == 0 {
        println!("No commits yet.");
        return;
    }

    println!("Commit History:\n");

    for commit_number in (1..=head).rev() {
        match load_commit(commit_number) {
            Ok(commit) => {
                println!("Commit {}", commit_number);

                for task in commit.tasks {
                    let status = if task.completed { "✓" } else { " " };
                    println!("[{}] {} - {}", status, task.id, task.text);
                }

                println!("----------------------");
            }
            Err(_) => println!("Failed to load commit {}", commit_number),
        }
    }
}
