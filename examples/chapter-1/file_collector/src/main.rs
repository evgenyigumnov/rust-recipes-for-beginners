// src/main.rs
mod collect;  // Declare the collect module
mod print;    // Declare the print module

use collect::get_files;  // Bring get_files function into scope
use print::display_files;  // Bring display_files function into scope

fn main() {
    let dir_path = "./sample_dir"; // Path to the directory containing files
    match get_files(dir_path) {
        Ok(files) => {
            display_files(files); // Pass the files to the print module for display
        },
        Err(e) => eprintln!("Error collecting files: {}", e),
    }
}