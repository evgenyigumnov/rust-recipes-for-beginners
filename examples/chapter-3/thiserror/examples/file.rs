use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileProcessingError {
    #[error("Failed to read file `{0}`: {1}")]
    ReadError(String, #[source] std::io::Error),
    #[error("Invalid data format in file `{0}`")]
    InvalidFormat(String),
    #[error("Unsupported file extension `{0}`")]
    UnsupportedExtension(String),
}

use std::fs::File;
use std::io::Read;

fn process_file(filename: &str) -> Result<(), FileProcessingError> {
    let mut file = File::open(filename)
        .map_err(|e| FileProcessingError::ReadError(filename.to_string(), e))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| FileProcessingError::ReadError(filename.to_string(), e))?;

    if !contents.starts_with("{") {
        return Err(FileProcessingError::InvalidFormat(filename.to_string()));
    }

    // Process contents...

    Ok(())
}

fn main() {
    match process_file("data.txt") {
        Ok(_) => println!("File processed successfully"),
        Err(e) => eprintln!("Error: {}", e),
    }
}