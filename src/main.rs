use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use std::env;
use jfs::Store;

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    name: String,
    completed: bool
}

fn main() {
    let mut cfg = jfs::Config::default();
    cfg.single = true;

    let db = Store::new_with_cfg("data", cfg).unwrap();

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Please specify a command.");
        std::process::exit(0);
    }

    // First argument should be disregarded - this is the path to the binary.
    // So take second argument as the command the user has chosen.
    let command: &String = &args[1];

    match command.as_str() {
        "ping" => println!("pong"),
        "show" => show(&db),
        "add" => add(db, get_name_from_args(&args)),
        _ => println!("That is not a valid command.")
    }
}

fn show(db: &Store) {
    let tasks: BTreeMap<String, Task> = db.all().unwrap();
    if tasks.len() == 0 {
        println!("No tasks found.");
    } else {
        println!("{:?}", tasks);
    }
}

fn get_name_from_args(args: &Vec<String>) -> String {
    let mut name = String::new();

    if args.len() == 2 {
        println!("Please enter a name for the new task.");
        std::process::exit(0);
    }

    for i in 2..args.len() {
        name.push_str(format!("{} ", &args[i]).as_ref());
    }

    name.trim().to_string()
}

fn add(db: Store, name: String) {
    let new_task = Task { name, completed: false };
    let id = db.save(&new_task).unwrap();
    println!("Added new task {id}.");
}