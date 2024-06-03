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

    let builtins = ["echo", "exit", "pwd", "cd", "type"];

    let mut cwd = std::env::current_dir().unwrap();

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
            "pwd" => {
                println!("{}", cwd.display());
            }
            "cd" => {
                let new_cwd = std::path::PathBuf::from(splits.next().unwrap());
                if new_cwd == std::path::Path::new("~") {
                    let home = std::path::PathBuf::from(env!("HOME"));
                    cwd = home;
                    std::env::set_current_dir(&cwd).unwrap();
                    continue;
                }
                let new_cwd_display = new_cwd.display().to_string();
                let Ok(new_cwd) = fs::canonicalize(new_cwd) else {
                    println!("cd: {}: No such file or directory", new_cwd_display);
                    continue;
                };
                if new_cwd.is_dir() {
                    cwd = new_cwd;
                    std::env::set_current_dir(&cwd).unwrap();
                } else {
                    println!("cd: {}: No such file or directory", new_cwd.display());
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
