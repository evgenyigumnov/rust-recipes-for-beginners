use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;

fn read_number_from_file(path: &str) -> Result<i32> {
    let mut file = File::open(path)
        .with_context(|| format!("Failed to open file at '{}'", path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context("Failed to read contents from file")?;
    let number = contents.trim().parse::<i32>()
        .context("Failed to parse number from file contents")?;
    Ok(number)
}
fn main() {
  match read_number_from_file("data.txt") {
    Ok(number) => println!("Number: {}", number),
    Err(e) => eprintln!("Error: {}", e),
  }
}