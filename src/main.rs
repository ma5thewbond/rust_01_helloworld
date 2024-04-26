use slug::slugify;
use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut input = String::new();
    println!("Enter text to modify: ");
    io::stdin().read_line(&mut input).unwrap();

    if args.len() == 1 {
        println!("Original: {input}");
    }

    if args[1] == "lowercase" {
        println!("lowercase: {}", input.to_lowercase());
    }

    if args[1] == "uppercase" {
        println!("UPPERCASE: {}", input.to_uppercase());
    }

    if args[1] == "no-spaces" {
        println!("nospaces: {}", input.replace(" ", ""));
    }

    if args[1] == "slugify" {
        let mut slug = input.clone();
        slug = slugify(slug);
        println!("slugify: {}", slug);
    }

    if args[1] == "reverse" {
        print!("esrever: ");
        for x in input.chars().rev() {
            print!("{x}");
        }
        println!("");
    }

    if args[1] == "trim" {
        println!("trim: {}", input.trim());
    }

    if args[1] == "all" {
        println!("Original: {input}");
        println!("lowercase: {}", input.to_lowercase());
        println!("UPPERCASE: {}", input.to_uppercase());
        println!("nospaces: {}", input.replace(" ", ""));
        let mut slug = input.clone();
        slug = slugify(slug);
        println!("slugify: {}", slug);
        print!("esrever: ");
        for x in input.chars().rev() {
            print!("{x}");
        }
        println!("");
        println!("trim: {}", input.trim());
    }
}
