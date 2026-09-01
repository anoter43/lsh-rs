use std::io::{self, Write};
use std::process::{Command, Stdio};

const BUILTINS: &[(&str, fn(&[String]) -> bool)] = &[
    ("cd", lsh_cd),
    ("help", lsh_help),
    ("exit", lsh_exit),
];

fn lsh_cd(args: &[String]) -> bool {
    match args.get(1) {
        None => eprintln!("lsh: expected argument to \"cd\""),
        Some(dir) => {
            if let Err(e) = std::env::set_current_dir(dir) {
                eprintln!("lsh: {}", e);
            }
        }
    }
    true
}

fn lsh_help(_args: &[String]) -> bool {
    println!("Stephen Brennan's LSH, ported to Rust");
    println!("Type program names and arguments, and hit enter.");
    println!("The following are built in:");
    for (name, _) in BUILTINS {
        println!("  {}", name);
    }
    println!("Use the man command for information on other programs.");
    true
}

fn lsh_exit(_args: &[String]) -> bool {
    false
}

fn lsh_launch(args: &[String]) -> bool {
    let child = Command::new(&args[0])
        .args(&args[1..])
        .spawn();

    match child {
        Ok(mut child) => {
            if let Err(e) = child.wait() {
                eprintln!("lsh: {}", e);
            }
        }
        Err(e) => eprintln!("lsh: {}", e),
    }
    true
}

fn lsh_execute(args: &[String]) -> bool {
    if args.is_empty() {
        return true;
    }
    for (name, func) in BUILTINS {
        if args[0] == *name {
            return func(args);
        }
    }
    lsh_launch(args)
}

fn lsh_split_line(line: &str) -> Vec<String> {
    line.split_whitespace().map(String::from).collect()
}

fn lsh_loop() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {}
            Err(e) => {
                eprintln!("lsh: {}", e);
                std::process::exit(1);
            }
        }

        let args = lsh_split_line(&line);
        if !lsh_execute(&args) {
            break;
        }
    }
}

fn main() {
    lsh_loop();
}
