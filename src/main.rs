use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use std::env;
use jfs::Store;
use nanoid::nanoid;
use colored::Colorize;

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
        "show" => show(&db),
        "add" => add(db, get_name_from_args(&args)),
        "complete" => complete(db, args[2].clone()),
        "delete" => delete(db, args[2].clone()),
        "help" => help(),
        _ => println!("{command} is not a valid command.")
    }
}

fn show(db: &Store) {
    let tasks: BTreeMap<String, Task> = db.all().unwrap();
    if tasks.len() == 0 {
        println!("No tasks found.");
    } else {
        for task in tasks.iter() {
            println!("{} {}: {}", completed_symbol(&task), &task.0, &task.1.name);
        }
    }
}

fn completed_symbol(task: &(&String, &Task)) -> colored::ColoredString {
    if task.1.completed {
        "✓".green()
    } else {
        "x".red()
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

    let alphabet: [char; 16] = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f'
    ];

    let id = nanoid!(6, &alphabet);
    let created = db.save_with_id(&new_task, &id).unwrap();
    println!("Added new task {created}.");
}

fn complete(db: Store, id: String) {
    let mut task: Task = db.get(&id).unwrap();
    if !task.completed {
        task.completed = true;
        db.save_with_id(&task, &id).unwrap();
        println!("Completed task {id}.");
    } else {
        println!("{id} is already complete.");
    }
}

fn delete(db: Store, id: String) {
    db.delete(&id).unwrap();
    println!("Deleted task {id}.");
}

fn help() {
    println!("{}", "Help:".bold());

    let help_commands = [
        ("show", "Show all tasks."),
        ("add <name...>", "Add a new task."),
        ("complete <id>", "Complete a task."),
        ("delete <id>", "Delete a task.")
    ];

    for command in help_commands.iter() {
        println!("{} - {}", command.0, command.1.italic());
    }
}