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
        if trimmed.starts_with("exit") {
            let code = trimmed.splitn(2, " ");
            let code = code.last().unwrap().parse::<i32>().unwrap();
            std::process::exit(code);
        }
        if !trimmed.is_empty() {
            println!("{}: command not found", trimmed)
        };
    }
}
