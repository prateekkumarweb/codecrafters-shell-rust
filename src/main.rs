use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let trimmed = input.trim();
        if !trimmed.is_empty() {
            println!("{}: command not found", trimmed)
        };
    }
}
