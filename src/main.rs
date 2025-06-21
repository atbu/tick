use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    name: String,
    completed: bool
}

fn main() {
    let test_task = r#"
        {
            "name": "Test Task",
            "completed": false
        }
    "#;

    let task: Task = serde_json::from_str(test_task).unwrap();

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
        "test" => println!("{:?}", task),
        _ => println!("That is not a valid command.")
    }
}
