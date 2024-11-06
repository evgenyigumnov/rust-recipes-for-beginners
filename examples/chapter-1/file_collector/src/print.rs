// src/print.rs
use std::fs;
use std::fs::DirEntry;

pub fn display_files(files: Vec<DirEntry>) {
    for file in files {
        let file_path = file.path();
        match fs::read_to_string(&file_path) {
            Ok(content) => {
                println!("File: {:?}", file_path);
                println!("Content:\n{}", content);
                println!("----------------------");
            }
            Err(e) => eprintln!("Error reading file {:?}: {}", file_path, e),
        }
    }
}

