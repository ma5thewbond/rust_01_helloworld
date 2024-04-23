use std::env;

fn main() {
    let mut args = env::args();
    args.next().unwrap();
    let greeting = args.next().unwrap_or("Hello".to_string());
    let name = args.next().unwrap_or("Rust".to_string());
    println!("{greeting}, {name}!");
}
