use anyhow::Result;
use std::fs::File;
use std::io::Read;

fn read_number_from_file(path: &str) -> Result<i32> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number = contents.trim().parse::<i32>()?;
    Ok(number)
}

fn main() {
    match read_number_from_file("data.txt") {
        Ok(number) => println!("Number: {}", number),
        Err(e) => eprintln!("Error: {}", e),
    }
}