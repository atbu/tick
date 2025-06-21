use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // First argument should be disregarded - this is the path to the binary.
    // So take second argument as the command the user has chosen.
    let command: &String = &args[1];
    
    if command == "ping" {
        println!("pong");
    }
}
