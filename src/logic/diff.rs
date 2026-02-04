use std::collections::HashMap;
use crate::storage::commit::load_commit;
use crate::models::task_model::Task;

pub fn diff(c1: usize, c2: usize) {
    let commit1 = match load_commit(c1) {
        Ok(c) => c,
        Err(_) => {
            println!("Commit {} not found.", c1);
            return;
        }
    };

    let commit2 = match load_commit(c2) {
        Ok(c) => c,
        Err(_) => {
            println!("Commit {} not found.", c2);
            return;
        }
    };

    let mut map1: HashMap<usize, Task> = HashMap::new();
    let mut map2: HashMap<usize, Task> = HashMap::new();

    for t in commit1.tasks {
        map1.insert(t.id, t);
    }

    for t in commit2.tasks {
        map2.insert(t.id, t);
    }

    println!("Diff between commit {} → {}\n", c1, c2);

    // Added tasks
    for (id, task) in &map2 {
        if !map1.contains_key(id) {
            println!("+ Added: {}. {}", task.id, task.text);
        }
    }

    // Removed tasks
    for (id, task) in &map1 {
        if !map2.contains_key(id) {
            println!("- Removed: {}. {}", task.id, task.text);
        }
    }

    // Status changes
    for (id, task1) in &map1 {
        if let Some(task2) = map2.get(id) && task1.completed != task2.completed{
                if task2.completed {
                    println!("✓ Completed: {}. {}", task2.id, task2.text);
                } else {
                    println!("↺ Marked incomplete: {}. {}", task2.id, task2.text);
                }
        }
    }
}
