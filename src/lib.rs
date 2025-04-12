mod builtins;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use std::io::Write;

pub fn run_builtin_command(command: &str, args: Vec<&str>) {
    match command {
        "" => {}
        "cd" => {
            if args.is_empty() {
                builtins::cd("~");
            } else {
                builtins::cd(args[0]);
            }
        }
        "cat" => {
            builtins::cat(args);
        }
        "pwd" => {
            builtins::pwd();
        }
        "ls" => {
            builtins::ls();
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

pub fn interactive_shell() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        eprintln!("No previous history.");
    }
    loop {
        let readline = rl.readline("turtle> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                parse_command(line.trim());
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                eprintln!("Error: {}", err)
            }
        }
    }
    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt")?;
    Ok(())
}
