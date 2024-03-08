mod builtins;
use std::io::Write;

use builtins::{ls, pwd};

pub fn run_builtin_command(command: &str, args: Vec<&str>) {
    match command {
        "" => {}
        "cat" => {
            builtins::cat(args);
        }
        "pwd" => {
            pwd();
        }
        "ls" => {
            ls();
        }
        "echo" => {
            builtins::echo(args);
        }
        "exit" => {
            std::process::exit(0);
        }
        _ => {
            let child = match std::process::Command::new(command).args(args).spawn() {
                Ok(output) => output,
                Err(e) => {
                    eprintln!("error: {}", e);
                    return;
                }
            };
            let output = child.wait_with_output().expect("failed to wait on child");
            std::io::stdout().write_all(&output.stdout).unwrap();
        }
    }
}

pub fn parse_command(command: &str) {
    let split_command = command.split_once(' ');
    match split_command {
        Some((cmd, args)) => {
            run_builtin_command(cmd, args.split(' ').collect());
        }
        None => {
            run_builtin_command(command, vec![]);
        }
    }
}

pub fn interactive_shell() {
    loop {
        print!("> ");
        let mut input = String::new();
        match std::io::stdout().flush() {
            Ok(_) => {}
            Err(_) => {
                eprintln!("error flushing stdout");
                continue;
            }
        }
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                parse_command(input.trim());
            }
            Err(_) => {
                eprintln!("error reading input");
                continue;
            }
        }
    }
}
