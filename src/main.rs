use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    name: String,
    completed: bool
}

fn main() {
    if !Path::new("tick.json").exists() {
        let empty = r#"[]"#;

        let mut file = File::create("tick.json").unwrap();
        file.write_all(empty.as_bytes()).unwrap();
    }

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
        "show" => show(),
        _ => println!("That is not a valid command.")
    }
}

fn show() {
    let tasks: Vec<Task> = get_tasks_from_file().unwrap();
    if tasks.len() == 0 {
        println!("No tasks found.");
    } else {
        println!("{:?}", tasks);
    }
}

fn get_tasks_from_file() -> Result<Vec<Task>, Box<dyn Error>> {
    let file = File::open("tick.json")?;
    let reader = BufReader::new(file);

    let tasks: Vec<Task> = serde_json::from_reader(reader)?;

    Ok(tasks)
}