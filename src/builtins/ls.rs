use std::fs;

pub fn ls() {
    let dir = std::env::current_dir();
    let paths = fs::read_dir(dir.unwrap()).unwrap();
    for path in paths {
        println!("{}", path.unwrap().file_name().to_str().unwrap());
    }
}
