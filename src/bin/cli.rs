use std::env;
use cotask::logic::{checkout, gc, init_repo, list_task, revert, show_log, task,merge};

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
            task::add_task(&args[2]);
        }

        "list" => list_task::list_tasks(),

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
            task::mark_done(id);

            let id: usize = args[2].parse().expect("Invalid task ID");
        }
        
        "log" => show_log::show_log(),

        "checkout" => {
            if args.len() < 3 {
                println!("Provide commit number.");
                return;
            }

            let num: usize = args[2].parse().expect("Invalid number");
            checkout::checkout(num);
        }

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
            if(args.len()<3){
                 println!("Provide commit number to merge.");
                return;
            }
            let num: usize = match args[2].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid commit number.");
                return;
            }
            };

            merge::merge(num);
        }




        _ => println!("Unknown command"),
    }
}
