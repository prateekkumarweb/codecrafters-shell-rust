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
        let mut splits = trimmed
            .split(' ')
            .map(|s| s.trim())
            .filter(|x| !x.is_empty());
        let command = splits.next();
        let Some(command) = command else {
            continue;
        };
        match command {
            "exit" => {
                let code = splits.next().unwrap();
                let code = code.parse::<i32>().unwrap();
                std::process::exit(code);
            }
            "echo" => {
                let args = splits.collect::<Vec<&str>>().join(" ");
                println!("{}", args);
            }
            command => {
                println!("{}: command not found", command)
            }
        }
    }
}
