use std::env;
use cotask::logic::{add_task, branch, checkout, diff, gc, init_repo,import,export,list_task, merge, revert, show_help, show_log,resolve, rebase,stash,tag};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: taskit <command>");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "init" => init_repo::init_repo(),

        "add" => {
            if args.len() < 3 {
                println!("Please provide task text.");
                return;
            }
            add_task::add_task(&args[2]);
        }

        "list" => list_task::list_tasks(),
        "export" => export::export_repo(),

        "import" => {
            if args.len() < 3 {
                println!("Provide backup file.");
                return;
            }
            import::import_repo(&args[2]);
        }

        "done" => {
            if args.len() < 3 {
                println!("Please provide task id.");
                return;
            }
            let id:usize = match args[2].parse() {
                Ok(n) => n,
                Err(_) =>{
                    println!("Invalid task ID");
                    return;
                }
                
            };
            add_task::mark_done(id);

        }
        
        "log" => show_log::show_log(),

        "checkout" => {
            if args.len() < 3 {
                println!("Provide branch, tag, or commit number.");
                return;
            }

            let input = &args[2];

            if let Ok(num) = input.parse::<usize>() {
                checkout::checkout_commit(num);
            } else {
                checkout::checkout_ref(input);
            }
        }

        "branch-delete" => {
            if args.len() < 3 {
                println!("Provide branch name to delete.");
                return;
            }
            branch::delete_branch(&args[2]);
        }

        "stash" => stash::stash(),
        "stash-pop" => stash::stash_pop(),

        "revert" => {
            if args.len() < 3 {
                println!("Provide commit number to revert.");
                return;
            }

            let num: usize = args[2].parse().expect("Invalid number");
            revert::revert(num);
        }

        "gc" => gc::run_gc(),

        "merge" => {
            if args.len() < 3 {
                println!("Provide branch name to merge.");
                return;
            }

            merge::merge_branch(&args[2]);
        }


        "branch" => {
            if args.len() < 3 {
                println!("Provide branch name.");
                return;
            }
            branch::create_branch(&args[2]);
        }

        "branches" => {
            branch::list_branches();
        }

        "diff" => {
            if args.len() < 4 {
                println!("Usage: diff <commit1> <commit2>");
                return;
            }

            let c1 = args[2].parse().unwrap_or(0);
            let c2 = args[3].parse().unwrap_or(0);

            diff::diff(c1, c2);
        }
        "--help" => {
            show_help::show_help();
        
        }

        "tag" => {
            if args.len() < 3 {
                println!("Provide tag name.");
                return;
            }
            tag::create_tag(&args[2]);
        }

        "tags" => {
            tag::list_tags();
        }


        "resolve" => {
            if args.len() < 4 {
                println!("Usage: resolve <task_id> done|undone");
                return;
            }

            let id: usize = args[2].parse().unwrap_or(0);
            let done = match args[3].as_str() {
                "done" => true,
                "undone" => false,
                _ => {
                    println!("Use 'done' or 'undone'");
                    return;
                }
            };

            resolve::resolve(id, done);
        }
        "rebase" => {
        if args.len() < 3 {
            println!("Provide target branch.");
            return;
        }

        rebase::rebase_onto(&args[2]);
    }

        _ => {
            println!("Unknown command.\n");
            show_help::show_help();  
        }

    }
}
