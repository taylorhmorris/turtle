pub fn cd(arg: &str) {
    let result = std::env::set_current_dir(arg);
    if let Err(e) = result {
        println!("cd: {}", e);
    }
}
