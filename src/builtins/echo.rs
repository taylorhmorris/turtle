pub fn echo(args: Vec<&str>) {
    for arg in args {
        print!("{} ", arg);
    }
    println!();
}
