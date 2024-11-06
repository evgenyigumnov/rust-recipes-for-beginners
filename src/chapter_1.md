# Chapter 1 - Cargo: Managing Rust Projects 

This chapter will introduce you to Cargo, the official package manager and build system for Rust. We will explore how Cargo simplifies project management, dependency handling, and code organization. By learning to configure projects with `Cargo.toml` files and organize complex projects into workspaces, you will gain a solid foundation for managing Rust projects of all sizes. Additionally, this chapter covers how to create libraries, publish crates, and effectively structure code using modules.

**Topics Covered:**

- Introduction to Cargo: Rust's package manager and build tool
- Initializing a new Rust project using Cargo
- Organizing projects with workspaces and configuring `Cargo.toml`
- Creating and managing libraries for code reuse
- Structuring Rust code with modules and file organization
- Managing dependencies and version requirements
- Publishing libraries to [https://crates.io](https://crates.io)

**Objectives:**



**Recipes:**

To help you master Cargo and efficiently manage your Rust projects, we'll walk through several key steps that will form the foundation of your workflow. From starting a new project to publishing your libraries, each section introduces practical skills you can immediately apply to your own development. These steps will guide you through initializing projects, organizing code, handling dependencies, and making your code accessible to others through publishing. Whether you're building applications or libraries, these recipes will equip you with the essential knowledge to streamline your development process with Cargo.

Now, let's dive into the core steps for managing Rust projects effectively.

1. **Initializing a New Project**: Learn how to create a new project using `cargo new` and understand the structure of a Cargo project.
2. **Organizing with Workspaces**: Set up multiple related packages under a single workspace to manage complex projects.
3. **Structuring Code with Modules**: Use modules to logically organize code, making it easier to navigate and maintain.
4. **Managing Dependencies**: Add external libraries (crates) and specify version constraints to ensure compatibility.
5. **Publishing Your Library**: Follow the steps to publish your crate on `crates.io`, making it available for others to use.


## Initializing a New Project

Ensure you have Rust installed. You can install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
After installing Rust, open a terminal and run the following command to check the version:

```bash
cargo --version
```
If you see the Cargo version, you are ready to create a new project. For example:
```plaintext
cargo 1.78.0 (54d8815d0 2024-03-26)
```

To create a new Rust project, use the `cargo new` command followed by the project name. For example, to create a project named `my_first_app`, run:
```bash
cargo new my_first_app
```
This command generates a new directory named `hello_world` containing the project files. The directory structure will look like this:
```
.
|-- Cargo.toml
`-- src
    `-- main.rs
```

The `Cargo.toml` file is the manifest file that contains metadata about the project, such as the project name, version, dependencies, and build configuration. 
The `src` directory contains the source code files, with `main.rs` as the entry point for the application.

**Cargo.toml**
```toml
# The package section contains metadata about the project.
[package]
# The name of the project or package. This is how it will
# be referred to in the Rust ecosystem (e.g., when 
# published to crates.io).
name = "my_first_app"

# The version of the project. Typically follows Semantic
# Versioning (Major.Minor.Patch).
version = "0.1.0"

# The Rust edition being used. Editions represent significant
# changes to the language and are backward-compatible.
# "2021" is the most recent stable edition at the time of writing.
edition = "2021"

# The dependencies section lists external libraries (crates) 
# that the project relies on. Currently, it's empty, but you
# would add   crates here as your project grows.
[dependencies]
```

**main.rs**
```rust
// The entry point of every Rust program is the `main` function.
// This function is where the program execution begins.
fn main() {
    // `println!` is a macro in Rust that prints text to
    // the console. The `!` indicates that `println!` is a macro 
    // and not a regular function. In this case, it prints the
    // string "Hello, world!" followed by a newline.
    println!("Hello, world!");
}
```

To build and run the project, run the following commands:

Navigate to the directory named 'my_first_app', where the Rust project is located.
```bash
cd my_first_app
```  
Compile and run the compiled binary in one step using Cargo.
``` bash
cargo run        
```


As a result, you should see the output `Hello, world!` printed to the console.
```
igumn@lenovo MINGW64 ~/rust-cookbook/chapter-1 (main)
$ cd my_first_app

igumn@lenovo MINGW64 ~/rust-cookbook/chapter-1/my_first_app (main)
$ cargo run
   Compiling my_first_app v0.1.0 (C:\Users\igumn\rust-cookbook\chapter-1\my_first_app)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target\debug\my_first_app.exe`
Hello, world!
```

Compiled binaries are stored in the `target` directory. The `debug` directory contains unoptimized binaries, while the `release` directory contains optimized binaries.

The `target` directory structure will look like this:
```
`-- target
    |-- CACHEDIR.TAG
    `-- debug
        |-- build
        |-- deps
        |   |-- my_first_app.d
        |   |-- my_first_app.exe
        |   `-- my_first_app.pdb
        |-- examples
        |-- incremental
        |   `-- my_first_app-3e3wegiqm8oe
        |       |-- s-h0rxonzjv6-1y95dpn-eiilw6nsgoq6rx463oobb7cq4
        |       |   |-- 1ljpz0ouu7lunv1m.o
        |       |   |-- 1q8ysy75gaoy5gti.o
        |       |   |-- 1trrj5rpv3xws0p2.o
        |       |   |-- 3dvdoff5vxj8b752.o
        |       |   |-- 4pb1sd7r5dltrao2.o
        |       |   |-- 5wjpui3l3nk7yfu.o
        |       |   |-- dep-graph.bin
        |       |   |-- query-cache.bin
        |       |   `-- work-products.bin
        |       `-- s-h0rxonzjv6-1y95dpn.lock
        |-- my_first_app.d
        |-- my_first_app.exe
        `-- my_first_app.pdb
```
You could go to the `target/debug` directory and run the binary directly:
```bash
igumn@lenovo MINGW64 ~/rust-cookbook/chapter-1/my_first_app (main)
$ cd target/debug

igumn@lenovo MINGW64 ~/rust-cookbook/chapter-1/my_first_app/target/debug (main)
$ ./my_first_app.exe
Hello, world!
```

To clean up the project and remove the generated binary, run:
```bash
igumn@lenovo MINGW64 ~/rust-cookbook/chapter-1/my_first_app (main)
$ cargo clean
     Removed 23 files, 3.1MiB total
```

## Organizing with Workspaces

When working on large projects in Rust, it's common to break down the codebase into multiple smaller packages or "crates"
that can be developed, tested, and reused independently. However, managing these crates can become cumbersome 
if done in isolation. Rust’s `cargo` offers a feature called **workspaces** to help manage multiple related packages 
under a single project. This chapter will walk you through organizing a project with workspaces, explaining the benefits 
and providing examples of how to set it up in the `Cargo.toml` file.


Let's begin with a step-by-step guide to set up a workspace.

**Step 1: Create a Workspace Directory**

First, create a directory for your workspace. This directory will contain the `Cargo.toml` file for the workspace and the individual package directories (crates).

```bash
mkdir my_workspace
cd my_workspace
```

**Step 2: Create Package Directories**

Next, create the individual package directories. Each package will have its own `Cargo.toml` and `src/` folder like any other Rust crate. For this example, we’ll create a core library that a CLI tool will use.

```bash
cargo new core_lib --lib
cargo new cli_app
```


**Step 3: Initialize a `Cargo.toml` for the Workspace**

At the root of the workspace, create a `Cargo.toml` file. This will be the **root manifest** file for the workspace. A workspace's `Cargo.toml` doesn't need to specify dependencies directly; instead, it will contain the configuration for the workspace and list its members (crates).

```toml
[workspace]
# The "resolver" key sets the version of the Cargo
# resolver to use. Resolver "2" is the new version 
# introduced in Rust 1.51, which  allows for more
# fine-grained control over dependencies.
resolver = "2"

# The "members" key defines the members of a 
# workspace. A workspace is a set of packages 
# (in this case, "core_lib" and "cli_app")
# that share a common Cargo.lock and output
# directory.Each member is treated as a separate
# package but can share dependencies and 
# configuration.
members = [
  # This is a package, likely containing the
  #core library functionality of the project.
  "core_lib",
  # This is another package, possibly implementing
  #  a command-line interface that uses "core_lib".
  "cli_app"    
]
```

The `members` is a list of relative paths to the crate directories within the workspace.

This file specifies that our workspace will consist of 2 packages: `core_lib` and `cli_app`.


After running these commands, your directory structure will look like this:

```
.
|-- Cargo.toml
|-- cli_app
|   |-- Cargo.toml
|   `-- src
|       `-- main.rs
`-- core_lib
    |-- Cargo.toml
    `-- src
        `-- lib.rs
```

**Step 4: Define Dependencies Between Crates**

Let’s make the `cli_app` package depend on the `core_lib`. This is where the power of workspaces comes into play: you can specify dependencies between crates within the workspace without needing to publish them externally.

In `cli_app/Cargo.toml`, add:

```toml
[dependencies]
core_lib = { path = "../core_lib" }
```

Now the CLI tool can use the functionality provided by `core_lib`.

**Step 5: Writing Some Code**

Simple functionality to `core_lib/src/lib.rs` (generated by `cargo new core_lib --lib`):

```rust
// This function takes two unsigned integers 
// (`usize`) as inputs and returns their sum.
pub fn add(left: usize, right: usize) -> usize {
  // Return the sum of `left` and `right`
  left + right
}

// This module contains unit tests for the 
// `add` function.
#[cfg(test)] // This annotation ensures that
             // the test module is only 
             // included when running tests.
mod tests {
  // Bring the `add` function from the parent
  // scope into this module.
  use super::*;

  // This test checks if the `add` function 
  // works as expected.
  #[test] // The `#[test]` attribute marks 
          // this function as a test case.
  fn it_works() {
    // Call the `add` function with arguments
    // 2 and 2, and store the result.
    let result = add(2, 2);
    // Assert that the result is equal to 4. 
    // If it is not, the test will fail.
    assert_eq!(result, 4);
  }
}
```

In the CLI tool (`cli_app/src/main.rs`), we can now use this function:

```rust
// Import the add function from the core_lib module
use core_lib::add;

fn main() {
  // Call the add function with arguments 3 and 5, and
  // store the result
  let result = add(3, 5);

  // Print the result of the addition in a formatted string
  println!("3 + 5 = {}", result);
}
```

**Step 6: Build and Test the Workspace**

Now that our workspace is set up, you can build and test all the crates at once using `cargo` commands from the workspace root.

**Build all crates**:

```bash
cargo build
```

**Test all crates**:

```bash
cargo test
```

You will see the output of the tests from `core_lib`:
```
igumn@lenovo MINGW64 ~/rust-cookbook/chapter-1/my_workspace (main)
$ cargo test
   Compiling cli_app v0.1.0 (C:\Users\igumn\rust-cookbook\chapter-1\my_workspace\cli_app)
   Compiling core_lib v0.1.0 (C:\Users\igumn\rust-cookbook\chapter-1\my_workspace\core_lib)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src\main.rs (target\debug\deps\cli_app-e76ea9dbea406c1b.exe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src\lib.rs (target\debug\deps\core_lib-bf8e6fa3ea4f729e.exe)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests core_lib

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

**Run individual crates**:

If you want to run a specific binary crate (like `cli_app`), you can do so by specifying the crate's directory:
```bash
cargo run -p cli_app
```
This command builds and runs the `cli_app` crate specifically.

```
igumn@lenovo MINGW64 ~/rust-cookbook/chapter-1/my_workspace (main)
$ cargo run -p cli_app
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
Running `target\debug\cli_app.exe`
3 + 5 = 8
```


## Structuring Code with Modules

When writing larger applications in Rust, it's crucial to structure your code in a way that promotes maintainability and clarity. Rust provides a powerful module system that allows you to logically organize your code. By grouping related functionality into modules, you create clear boundaries between different parts of your application, making it easier to navigate and manage.

In this chapter, we'll explore how to use Rust's module system by creating a simple project that collects files from a directory and prints each file's path and contents. We'll structure this project using two modules: `collect` and `print`. The `collect` module will handle gathering files, and the `print` module will be responsible for displaying the paths and content.

**Project Structure**

    Here's the folder structure of the project we'll be working on:

```
.
|-- Cargo.lock
|-- Cargo.toml
|-- sample_dir
|   |-- file1.txt
|   `-- file2.txt
`-- src
    |-- collect.rs
    |-- main.rs
    `-- print.rs
```

Easy to start a new project with Cargo:
```bash
cargo new file_collector
cd file_collector
```

We'll start by creating the modules and structuring the `main.rs` file.

**main.rs: Declaring Modules**

In Rust, modules can either be defined in the same file or in separate files. In our case, we'll define them in separate files for better organization.

```rust
// src/main.rs
// Declare the collect module, where the logic for 
// gathering files will reside
mod collect;  

// Declare the print module, where the logic for
// displaying files will reside
mod print;    

// Import the get_files function from the collect 
// module to use in this file
use collect::get_files;  

// Import the display_files function from the print 
// module to use in this file
use print::display_files;  

// The main function, which serves as the entry point 
// of the program
fn main() {
    // Define the path to the directory where the files
    // are located
    let dir_path = "./sample_dir"; 

    // Use match to handle the result from 
    // get_files function
    match get_files(dir_path) {
        // If the function successfully returns a list 
        // of files, pass them to display_files 
        // for printing
        Ok(files) => {
            display_files(files); 
        },
        // If there is an error collecting files, print
        // an error message to stderr
        Err(e) => eprintln!("Error collecting files: {}", e),
    }
}
```

Here, we declare the `collect` and `print` modules, which are stored in separate files (`collect.rs` and `print.rs`). The `get_files` function from the `collect` module is used to gather the files, and the `display_files` function from the `print` module is used to print the file paths and contents.

**collect.rs: Gathering Files**

The `collect` module is responsible for collecting all files from the specified directory. We'll use Rust's `std::fs` library to read the directory contents.

```rust
// src/collect.rs
use std::fs::{self, DirEntry};  
// Importing the  necessary modules for working
// with the filesystem
use std::path::Path;            
// Importing the Path struct to work with file paths
use std::io;                    
// Importing the io module for handling
// input/output errors

// This function retrieves a list of files from a 
// given directory path. It takes a directory path 
// as a string and returns either a vector 
// of `DirEntry` (file entries) or an `io::Error`.
pub fn get_files(dir_path: &str) 
    -> Result<Vec<DirEntry>, io::Error> {
    let mut files = Vec::new();  
    // Initialize an empty 
    // vector to store the files

    // Read the directory contents using the provided 
    // directory path. `fs::read_dir` returns an iterator
    // over the directory entries (files and directories).
    for entry in fs::read_dir(Path::new(dir_path))? {
        let entry = entry?;  
        // Unwrap the Result, retrieving the DirEntry or 
        // returning an error if there's one.

        // Check if the entry is a file. If it's a file, 
        // add it to the vector.
        if entry.path().is_file() {
            files.push(entry);
        }
    }

    // Return the vector of files if everything goes well.
    Ok(files)
}
```

This module defines a `get_files` function, which takes the path to a directory and returns a vector of `DirEntry` objects representing the files found. It uses the `std::fs::read_dir` function to iterate over the directory's contents and filter out files.

**print.rs: Displaying Files and Contents**

The `print` module is responsible for printing the paths of the files and their contents to the console.

```rust
// src/print.rs
use std::fs; 
// Importing the filesystem module to interact with files.
use std::fs::DirEntry; 
// Importing DirEntry to represent individual directory 
// entries (files or subdirectories).

// This function takes a vector of DirEntry (representing
// files) and displays the content of each file.
pub fn display_files(files: Vec<DirEntry>) {
    // Iterate over each file in the given vector of files
    for file in files {
        // Get the file path from the DirEntry
        let file_path = file.path();

        // Attempt to read the contents of the file as a string
        match fs::read_to_string(&file_path) {
            // If successful, print the file path and its contents
            Ok(content) => {
                println!("File: {:?}", file_path); 
                // Print the file path
                println!("Content:\n{}", content); 
                // Print the file's content
                println!("----------------------"); 
                // Separator for readability
                // between files
            }
            // If there is an error while reading the file, print 
            // the error message
            Err(e) => eprintln!("Error reading file {:?}: {}",
                                file_path, e),
        }
    }
}
```

This module defines the `display_files` function, which takes a vector of `DirEntry` objects and iterates over them. For each file, it prints the file path and reads its contents using `fs::read_to_string`. If there's an error reading the file, an error message is printed.

**Putting It All Together**

To run this project, you'll need to create some sample files in a directory (`sample_dir`) for the program to process. Once the project is structured as shown above, run it with `cargo run`. The program will collect all files from the specified directory, print their paths, and display their contents.
    
**Example Output**

For example, if the directory contains the following files:

```
.
|-- sample_dir
|   |-- file1.txt
|   `-- file2.txt
```

With content:

- `file1.txt`: "Hello, World!"
- `file2.txt`: "Rust is awesome!"

The output would look like this:

```
igumn@lenovo MINGW64 ~/rust-cookbook/chapter-1/file_collector (main)
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\file_collector.exe`
File: "./sample_dir\\file1.txt"
Content:
Hello, World!
----------------------
File: "./sample_dir\\file2.txt"
Content:
Rust is awesome!
----------------------
```

## Managing Dependencies in Rust

In this chapter, we’ll explore how to manage external libraries (also known as crates) in Rust projects. We’ll discuss how to specify dependencies, how to manage version constraints for compatibility, and how to use features from crates to unlock additional functionality. We'll be working with the `rand` crate as an example throughout this chapter, progressing from a simple case to a more complex setup involving features.


**1. Adding Dependencies**

In Rust, dependencies are specified in the `Cargo.toml` file, which is automatically generated when you create a new project using `cargo new`. The dependencies section allows you to declare which external crates your project will use.

**Example: Adding `rand` crate**

We will start by adding the `rand` crate, which provides utilities for generating random numbers. The latest version can be found on [crates.io](https://crates.io/crates/rand), but let’s add a specific version to ensure compatibility with our code.

In `Cargo.toml`:
```toml
[dependencies]
rand = "0.8.5"
```

Here, `rand = "0.8.5"` indicates that we want version 0.8.5 of the `rand` crate. The version constraint `"0.8.5"` ensures that cargo will use version 0.8.5, but it also allows updates within the same minor version (for example, to 0.8.6, 0.8.7, etc.), while preventing upgrades to version 0.9 or beyond.

**Example Project Layout**

Here’s how the directory structure will look for this simple project:

```
.
|-- Cargo.toml
`-- src
    `-- main.rs
```

In `src/main.rs`, we can now use the `rand` crate:

```rust
// Import the Rng trait from the rand crate to 
// generate random numbers
use rand::Rng;

// Entry point of the program
fn main() {
    // Call the function to generate a random number
    // between 1 and 100
    let random_number = generate_random_number(1, 100);

    // Print the generated random number
    println!("Random number: {}", random_number);
}

// Function to generate a random number between the
// given range (min and max inclusive)
fn generate_random_number(min: u8, max: u8) -> u8 {
    // Create a random number generator object 
    // using thread_rng()
    let mut rng = rand::thread_rng();

    // Generate a random number in the specified 
    // range (inclusive of both min and max)
    rng.gen_range(min..=max)
}

// Unit tests module for testing the 
// functions in the program
#[cfg(test)]  
// Ensures the module is only compiled during testing
mod tests {
    // Import functions from the parent scope for testing
    use super::*;

    // Test function to ensure the random number 
    // is within the expected range
    #[test] // Annotation to mark this as a test function
    fn test_random_number_range() {
        // Generate a random number between 1 and 100
        let random_number = generate_random_number(1, 100);

        // Assert that the random number is within
        // the range (inclusive)
        assert!(random_number >= 1 && random_number <= 100);
    }
}
```

This small program generates a random number between 1 and 100 using the `rand` crate.

And a test to check if the random number falls within the expected range.

**2. Managing Versions**

Rust allows you to define version constraints to maintain compatibility between different crates. Here's a quick breakdown of how versioning works in `Cargo.toml`:

- **Exact version**: `rand = "0.8.5"` ensures that the exact version 0.8.5 is used.
- **Caret requirement**: `rand = "^0.8.5"` allows any compatible version up to, but not including, 0.9.0.
- **Tilde requirement**: `rand = "~0.8.5"` restricts updates to patch versions only (e.g., 0.8.6).
- **Wildcard requirement**: `rand = "0.8.*"` allows patch version updates (e.g., 0.8.5 to 0.8.6) but restricts updates to minor versions.


**3. Using Crate Features**

Crates can come with optional features that are disabled by default. These features unlock additional functionality, allowing you to customize your dependency without pulling in unnecessary code.

**Example: `rand` Features**

The `rand` crate comes with several optional features such as `std` (for working in a `no_std` environment), `small_rng` (a faster but less secure random number generator), and others. You can enable features in the `Cargo.toml` file.

Let’s say we want to use the `small_rng` feature for a more lightweight random number generator.

In `Cargo.toml`:
```toml
[dependencies]
rand = { version = "0.8.5", features = ["small_rng"] }
```

In `src/main.rs`:

```rust
// Importing necessary traits and structs from
// the `rand` crate.
use rand::Rng; 
// Trait for random number generation methods.
use rand::rngs::SmallRng; 
// A small, fast pseudo-random number generator.
use rand::SeedableRng; 
// Trait that allows creating a random number 
// generator from a seed.

fn main() {
    // Create an instance of `SmallRng`, seeded
    // from the system's entropy source.
    // `from_entropy()` provides a convenient 
    // way to initialize the RNG with 
    // randomness from the system.
    let mut rng = SmallRng::from_entropy();

    // Generate a random number of type `u8` 
    // in the range 1 to 100 (inclusive
    // of 1 and exclusive of 101).
    let random_number: u8 = rng.gen_range(1..101);

    // Print the random number to the console.
    println!("Random number with SmallRng: {}", random_number);
}

#[cfg(test)] 
// This marks the following module as a test 
// module, which will only be compiled 
// in test mode.
mod tests {
    use super::*; // Import all items from the
    // parent scope for testing.
    use rand::rngs::SmallRng; // Importing `SmallRng`
                              // for use in the test.
    use rand::SeedableRng; // Importing `SeedableRng` 
                           // to seed the RNG.

    #[test] 
    // This attribute marks the following function as a test.
    fn test_random_number_with_small_rng() {
        // Create an instance of `SmallRng` seeded 
        // from system entropy, just like
        // in the main function.
        let mut rng = SmallRng::from_entropy();

        // Generate a random number of type `u8` in 
        // the range 1 to 100.
        let random_number: u8 = rng.gen_range(1..101);

        // Assert that the generated number falls
        // within the expected range 
        // (inclusive 1, exclusive 101).
        assert!(random_number >= 1 && random_number <= 100);
    }
}
```

Here, we use `SmallRng`, a random number generator that is faster but trades off some cryptographic security, which is acceptable for non-secure use cases.


**4. Adding Features to Your Own Library**

Rust allows you to define features in your own crate, giving consumers of your library the ability to enable or disable certain functionality. For instance, you can offer different serialization options such as `serde_json` or `bincode`, and users of your library can choose which one they need.

**Example: Defining Features for `serde_json` and `bincode` Serialization**

In this section, we will create a simple library that defines a `User` struct and provides two optional features: one for `serde_json` serialization and another for `bincode` serialization. We'll show how to set up the project, define the features, and implement conditional compilation based on which features are enabled.

**Project Setup**

Let’s create a new library project named `my_user_library`:

```bash
cargo new my_user_library --lib
cd my_user_library
```

The directory structure of the project will look like this:

```
.
|-- Cargo.toml
`-- src
    `-- lib.rs
```

**Step 1: Adding Features in `Cargo.toml`**

Now, we’ll add features to support `serde_json` and `bincode` serialization. Features are defined in the `[features]` section of `Cargo.toml`. Here’s how you can specify optional dependencies and features:

In `Cargo.toml`:

```toml
[package]
name = "my_user_library" 
version = "0.1.0"        
edition = "2021"         

[dependencies]
# This section lists the external crates (libraries)
# that your project depends on.

# Serde is a popular serialization/deserialization 
# framework in Rust. It's made optional because it 
# may not be needed unless specific features are enabled.
serde = { version = "1.0.210", features = ["derive"],
   optional = true }

# serde_json is a library to handle JSON 
# serialization/deserialization using Serde.
# It's also optional, to allow inclusion only
# when the corresponding feature is used.
serde_json = { version = "1.0.128", optional = true }

# bincode is another serialization format, often used
# for compact binary encoding.
# Like the others, it's made optional and tied to a feature.
bincode = { version = "2.0.0-rc.3", optional = true }

[features]
# This section defines the optional features that the crate
# supports.

# 'default' specifies which features should be enabled 
# by default.
# In this case, the `serde_json_support` feature is 
# enabled by default.
default = ["serde_json_support"]

# This feature includes support for JSON 
# serialization/deserialization.
# It pulls in both `serde` (with the derive feature)
# and `serde_json`.
serde_json_support = ["serde", "serde_json"]

# This feature enables support for bincode
# serialization/deserialization.
# It pulls in `serde` and `bincode` when this
# feature is enabled.
bincode_support = ["serde", "bincode"]
```

Here, we declare the optional dependencies `serde`, `serde_json`, and `bincode`, and create two features:
`serde_json_support`: Enables `serde_json` serialization.
`bincode_support`: Enables `bincode` serialization.
The `default` feature is set to `serde_json_support`, meaning that when no features are specified, `serde_json_support` will be enabled by default.

**Step 2: Conditional Compilation with Features**

You could use #[cfg(feature = "feature_name")] attribute to conditionally compile code based on the
enabled features. This allows you to include or exclude specific code blocks depending on the features enabled by the user.

In `src/lib.rs`:

```rust
// Conditionally includes the `serde` crate for JSON support if
// the `serde_json_support` feature is enabled.
#[cfg(feature = "serde_json_support")]
use serde::{Serialize, Deserialize};

// Conditionally includes the `bincode` crate for binary encoding 
// support if the `bincode_support` feature is enabled.
#[cfg(feature = "bincode_support")]
use bincode::{config, Decode, Encode};

// The `User` struct will derive different traits depending on 
// the enabled features.
// If the `serde_json_support` feature is enabled, it will derive 
// `Serialize` and `Deserialize` for JSON serialization.
// If the `bincode_support` feature is enabled, it will derive 
// `Decode` and `Encode` for binary encoding.
#[cfg_attr(feature = "serde_json_support", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bincode_support", derive(Decode, Encode))]
pub struct User {
    pub name: String,  // A public field representing 
                       // the user's name (String type).
    pub age: u8,       // A public field representing the
                       // user's age (unsigned 8-bit integer).
}

// If JSON support is enabled, implement a method to 
// convert the `User` struct to a JSON string.
#[cfg(feature = "serde_json_support")]
impl User {
    pub fn to_json(&self) -> String {
        // Use `serde_json` to serialize the `User` struct to a JSON string.
        serde_json::to_string(self).expect("Failed to serialize to JSON")
    }
}

// If binary encoding support is enabled, implement a method to 
// convert the `User` struct to a bincode vector.
#[cfg(feature = "bincode_support")]
impl User {
    pub fn to_bincode(&self) -> Vec<u8> {
        // Configure `bincode` using a standard configuration.
        let config = config::standard();
        // Encode the `User` struct to a binary format using `bincode`.
        bincode::encode_to_vec(self, config).expect("Failed to serialize to bincode")
    }
}

// A test module to test the `User` struct and its serialization functions.
mod tests {
    use super::User;  // Import the `User` struct for testing.

    #[test]  // Define a unit test function.
    fn test_user_struct() {
        // Create a `User` instance with the name "Alice" and age 30.
        let user = User {
            name: "Alice".to_string(),
            age: 30,
        };

        // If JSON support is enabled, check if the JSON 
        // serialization works as expected.
        #[cfg(feature = "serde_json_support")]
        assert_eq!(user.to_json(), r#"{"name":"Alice","age":30}"#);

        // If bincode support is enabled, check if the bincode 
        // serialization works as expected.
        #[cfg(feature = "bincode_support")]
        assert_eq!(user.to_bincode(), vec![5, 65, 108, 105, 99, 101, 30]);
    }
}

```

For launching test, you can use the following commands:
```bash
cargo test --features "serde_json_support bincode_support"
```

**Step 4: Example Usage with Features**

```bash
cargo new my_user_app
cd my_user_app
```

To use your library with one of the features enabled, the user needs to specify the feature in their `Cargo.toml`:

**Example 1: Using `serde_json` for Serialization**

In the user’s `Cargo.toml`:

```toml
[dependencies]
my_user_library = { path = "../my_user_library", features = ["serde_json_support"] }
```

In their code:

```rust
use my_user_library::User;

fn main() {
    let user = User{ name: "Alice".to_string(), age: 30 };
    let json = user.to_json();
    println!("Serialized to JSON: {}", json);
}
```

To run the code, the user would use
    
```bash
cargo run
```

The output would be:
```
Serialized to JSON: {"name":"Alice","age":30}
```

**Example 2: Using `bincode` for Serialization**

In the user’s `Cargo.toml`:

```toml
[dependencies]
my_user_library = { path = "../my_user_library", features = ["bincode_support"] }
```

In their code:

```rust
use my_user_library::User;

fn main() {
    let user = User{ name: "Alice".to_string(), age: 30 };
    let encoded = user.to_bincode();
    println!("Serialized to bincode: {:?}", encoded);
}
```



## Publishing Your Library on crates.io

In this chapter, we will walk through the process of publishing a Rust library on [crates.io](https://crates.io), making it available for others to use. To make this tutorial practical, we’ll create a simple library that helps measure the time it takes to execute a block of code. The library will allow users to mark the start and end points of a block of code and print the duration in nanoseconds.

**Step 1: Create a New Rust Library**

First, let’s create a new Rust library project. In your terminal, run:

```bash
cargo new --lib time_measure
cd time_measure
```

This creates a new Rust library in the folder `time_measure`. By default, Cargo will set up the project with a `src/lib.rs` file, where we will write our library code.

**Step 2: Write the Code for the Library**

Let’s implement a simple library that helps measure the time taken to execute a block of code. We will use Rust’s `std::time::Instant` for this.

In `src/lib.rs`, add the following code:

```rust
use std::time::Instant;

/// Starts the timer and returns the current time.
pub fn start_timer() -> Instant {
    Instant::now()
}

/// Stops the timer and prints the elapsed time in nanoseconds.
pub fn end_timer(start: Instant) {
    let duration = start.elapsed();
    println!("Execution time: {} nanoseconds", duration.as_nanos());
}
```

Here’s what we did:
- The `start_timer` function starts the timer by returning the current time using `Instant::now()`.
- The `end_timer` function calculates the elapsed time from the start time, then prints the duration in nanoseconds.

**Step 3: Test the Library**

Let’s test our library by writing a small example. Create a new file named `examples/demo.rs` and add the following code:

```rust
use time_measure::{start_timer, end_timer};

fn main() {
    let start = start_timer();
    
    // Example: Code block to measure
    for i in 0..100000 {
        let _ = i * i;
    }
    
    end_timer(start);
}
```

Run the example to see the time measurement in action:

```bash
cargo run --example demo
```

You should see the output showing the execution time in nanoseconds.

```
Execution time: 4097639 nanoseconds
```

**Step 4: Prepare the `Cargo.toml` File**

Before we can publish the crate to `crates.io`, we need to update the `Cargo.toml` file with the necessary metadata.

Open `Cargo.toml` and modify it as follows:

```toml
[package]
name = "time_measure"
version = "0.1.0"
authors = ["Your Name <your_email@example.com>"]
edition = "2021"
description = "A simple library to measure the execution time of code blocks"
license = "MIT"
repository = "https://github.com/your_username/time_measure"
```

Make sure to:
- Replace `Your Name` and `your_email@example.com` with your actual name and email.
- Set up a repository on GitHub or any other platform, and add its URL to the `repository` field.
- Add a short description of your library and specify the license (MIT, in this case).

**Step 5: Create an Account on `github.com` and `crates.io`**

If you don’t have an account on `github.com`, create one by going to [github.com](https://github.com) and signing up. 

To publish your library on `github.com`, you need to create a new repository and push your code to it.
First, create a new repository on GitHub by following these steps:

1. Click on the `+` icon in the top right corner and select `New repository`.
2. Enter the repository name (e.g., `time_measure`).
3. Add a description and choose the repository will be public.
4. Click on `Create repository`.
5. Follow the instructions to push your code to the repository.

Before pushing the code to the repository, you need to set up your SSH key. 
If you haven’t done this before, you can generate an SSH key by running the following command in your terminal:
```bash
ssh-keygen
```

You will be prompted to enter a file to save the key. Press `Enter` to use the default location (`/root/.ssh/id_rsa`). You can also set a passphrase for the key, but it’s optional.

```
Generating public/private rsa key pair.
Enter file in which to save the key (/root/.ssh/id_rsa):
Enter passphrase (empty for no passphrase):
Enter same passphrase again:
Passphrases do not match.  Try again.
Enter passphrase (empty for no passphrase):
Enter same passphrase again:
Your identification has been saved in /root/.ssh/id_rsa
Your public key has been saved in /root/.ssh/id_rsa.pub
The key fingerprint is:
SHA256:WNLRbQnuSgY1TVDmtQs+ghMiWzpI7HlT6t/l4LW7CkE root@server
The key's randomart image is:
+---[RSA 3072]----+
|        =*=o..   |
|.      o *o.+.   |
| o. oE+ o +..    |
|o..=+. B o . .   |
|.o++ .+ S + .    |
|  o.. .+ o .     |
|   . . ..o       |
|    . + = .      |
|     . +.=o      |
+----[SHA256]-----+
```

After generating the SSH key, you need to add it to your GitHub account.

1. Go to your GitHub account settings.
2. Click on `SSH and GPG keys`.
3. Click on `New SSH key`.
4. Copy the contents of the public key file (`id_rsa.pub`) and paste it into the key field.

To display the public key, you can use the following command:
```bash
cat .ssh/id_rsa.pub
```

Example of the public key:

```
ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQC/... root@server
```

After adding the SSH key to your GitHub account, you can push the code to the repository:

```bash
cd time_measure
echo "# time_measure" >> README.md
git init
git config user.email "you@example.com"
git config user.name "Your Name"
git add README.md
git commit -m "first commit"
git branch -M main
git remote add origin git@github.com:your_username/time_measure.git
git push -u origin main
git add Cargo.toml
git add src/lib.rs
git add examples/demo.rs
git commit -m "source code"
git push
```

After that you need to signing up via GitHub account on `crates.io` by going to [crates.io](https://crates.io).
After signing up, you need to generate an API token to publish your crate.

1. Go to your account settings and click “New Token” under API Tokens tab. (Select options in "Scopes" section: "publish-new", "publish-update")
2. Copy the generated token.
3. Add email address to your account settings. And verify it. Without verified email you can't publish crate.

**Step 6: Publish the Crate**

Now that everything is ready, it’s time to publish your library.

1. Authenticate Cargo with your `crates.io` account using the token:

```bash
cargo login <your-api-token>
```

   Replace `<your-api-token>` with the token you copied from your `crates.io` account.

2. To publish your crate to `crates.io`, run:

```bash
cargo publish
```

Once this is complete, your library will be available on `crates.io`, and other developers can use it by adding the following line to their `Cargo.toml`:

```toml
time_measure = "0.1.0"
```
