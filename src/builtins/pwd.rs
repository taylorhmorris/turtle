pub fn pwd() {
    let dir = std::env::current_dir();
    if let Ok(dir_str) = dir {
        if let Some(dir_str) = dir_str.to_str() {
            println!("{}", dir_str);
        }
    }
}
