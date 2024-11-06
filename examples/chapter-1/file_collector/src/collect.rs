// src/collect.rs
use std::fs::{self, DirEntry};
use std::path::Path;
use std::io;

pub fn get_files(dir_path: &str) -> Result<Vec<DirEntry>, io::Error> {
    let mut files = Vec::new();
    
    // Read the directory contents
    for entry in fs::read_dir(Path::new(dir_path))? {
        let entry = entry?;
        if entry.path().is_file() {
            files.push(entry);
        }
    }
    
    Ok(files)
}