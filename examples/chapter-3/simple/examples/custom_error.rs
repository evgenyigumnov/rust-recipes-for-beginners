use std::fs::File;
use std::io;
use std::io::Read;
use std::num::ParseIntError;

#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(ParseIntError),
}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> MyError {
        MyError::Io(error)
    }
}

impl From<ParseIntError> for MyError {
    fn from(error: ParseIntError) -> MyError {
        MyError::Parse(error)
    }
}

fn read_number_from_file(path: &str) -> Result<i32, MyError> {
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;  // Errors converted to MyError
    let number = contents.trim().parse()?;             // Parses string to number
    Ok(number)
}

fn main() {
    match read_number_from_file("data.txt") {
        Ok(number) => println!("Number: {}", number),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}