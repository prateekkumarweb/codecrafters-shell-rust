use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    let path_env = env!("PATH");
    let paths = path_env.split(':');

    let execs = paths
        .map(fs::read_dir)
        .filter_map(|dir| dir.ok())
        .flat_map(|dir| {
            dir.filter_map(|p| p.ok())
                .filter(|p| p.path().is_file())
                .filter_map(|p| p.file_name().to_str().map(|f| (f.to_owned(), p.path())))
        })
        .collect::<HashMap<_, _>>();

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
        let builtins = ["echo", "exit", "type"];
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
            "type" => {
                let command = splits.next().unwrap();
                if builtins.contains(&command) {
                    println!("{} is a shell builtin", command);
                } else if let Some(p) = execs.get(command) {
                    println!("{} is {}", command, p.as_os_str().to_str().unwrap());
                } else {
                    println!("{} not found", command);
                }
            }
            command => {
                if let Some(p) = execs.get(command) {
                    let args = splits.collect::<Vec<&str>>();
                    let _ = std::process::Command::new(p).args(&args).status().unwrap();
                } else {
                    println!("{}: command not found", command);
                }
            }
        }
    }
}
