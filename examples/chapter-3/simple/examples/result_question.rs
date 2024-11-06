use std::fs::File;
use std::io::{self, Read};

fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;          // Propagates error if file can't be opened
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;       // Propagates error if reading fails
    Ok(contents)
}

fn main() {
    match read_file_contents("data.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}