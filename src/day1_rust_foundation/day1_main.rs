use std::env;

#[allow(dead_code)]
fn day1_main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "add" => add_task(&args[2]),
            "list" => list_task(),
            _ => println!("Invalid command"),
        }
    }
}

#[allow(dead_code)]
fn add_task(task: &str) {
    println!("Task added: {}", task);
}

#[allow(dead_code)]
fn list_task() {
    println!("Task list: ");
    println!("1. Learn Rust.");
}