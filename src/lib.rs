mod builtins;
use builtins::{ls, pwd};

pub fn run_builtin_command(command: &str, args: Vec<&str>) {
    match command {
        "pwd" => {
            pwd();
        }
        "ls" => {
            ls();
        }
        "echo" => {
            builtins::echo(args);
        }
        _ => {
            println!("unknown command:");
            println!("command: {:?}", command);
            println!("args: {:?}", args);
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
