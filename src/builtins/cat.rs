use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn cat(files: Vec<&str>) {
    for filename in files {
        let path = Path::new(filename);
        let file = File::open(path).unwrap();
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            println!("{}", line.unwrap());
        }
    }
}
