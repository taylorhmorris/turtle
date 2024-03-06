use clap::Parser;
use turtle::{interactive_shell, parse_command};

/// A simple cross-platform shell
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// input file
    #[arg(num_args(0..))]
    input: Vec<String>,

    #[arg(short)]
    command: Option<String>,
}

fn main() {
    let args = Args::parse();

    if args.command.is_some() {
        parse_command(&args.command.unwrap());
    } else if args.input.is_empty() {
        interactive_shell();
    } else {
        println!("input file: {:?}", args.input);
    }
}
