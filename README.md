# Rust Recipes for Beginners
## A book for beginners in Rust, not beginners in programming
| Chapter Titles                                                      | Concepts & Technologies Covered                                       | Status      |
|---------------------------------------------------------------------|-----------------------------------------------------------------------|-------------|
| **[Introduction](./src/chapter_0.md)**                              | Cargo, Project Initialization, Workspaces, Modules, Dependencies      | Done        |
| **[Chapter 1: Cargo: Managing Rust Projects](./src/chapter_1.md)**  | Cargo, Project Initialization, Workspaces, Modules, Dependencies      | Done        |
| **[Chapter 2: Functional Programming in Rust](./src/chapter_2.md)** | Iterators, Closures, Mapping, Filtering, Folding, Enumerating         | Done        |
| **[Chapter 3: Error Handling](./src/chapter_3.md)**                 | Result & Option, `?` Operator, `anyhow`, `thiserror`, Custom Errors   | Done        |
| **[Chapter 4: Command Line Applications](./src/chapter_4.md)**      | `clap`, CLI Development, Argument Parsing, Environment Variables             | Done        |
| **[Chapter 5: Logging and Monitoring](./src/chapter_5.md)**         | `log`, `tracing`, Diagnostics, Monitoring Tools                              | Done        |
| **Chapter 6: Serialization and Deserialization**                   | Serde, JSON, YAML, TOML, CSV, Bincode, Custom Serialization                  | In progress |
| **Chapter 7: Memory Management and Smart Pointers**                | Ownership, `Box`, `Rc`, `Arc`, `Mutex`, `RefCell`, `Weak`, `Drop`, `unsafe`  |             |
| **Chapter 8: Design Patterns**                                     | Creational, Structural, Behavioral Patterns, Traits, Enums, Smart Pointers             |             |
| **Chapter 9: Asynchronous Programming**                            | `async`/`await`, Tokio, Futures, Async I/O, Task Management, Concurrency, Crossbeam    |             |
| **[Chapter 10: Working with Databases**](./src/chapter_10.md)                           | SQLite, MySQL, PostgreSQL, SQLx, SeaORM, Diesel, Async Database Operations            | In progress|
| **Chapter 11: Web Development**                                    | Axum, Actix, `reqwest`, `ureq`, `hyper`, HTTP Clients, Servers, APIs, Asynchronous Web|             |
| **Chapter 12: Building AI Pipelines with LangChain-Rust**          | LLMs, Embeddings, Vector Stores, Document Loaders, AI Integration                     |             |
| **Chapter 13: Integrations with Other Languages**                  | UniFFI, WebAssembly, FFI, Cross-Language Support, Memory Safety in FFI                |             |
| **[Chapter 14: Cryptography](./src/chapter_14.md)**                 | Hashing (SHA), Encryption (RSA, AES), Digital Signatures, `ring`, Key Management      | In progress |

## Examples and Source Code for Each Chapter

https://github.com/evgenyigumnov/rust-recipes-for-beginners/tree/main/examples

## Contributing
I would love to see contributions from the community. Simple steps to contribute:
1. Do fork
2. Write your list of recipes for the chapter. Ask the AI to expand on it. Ask the AI to describe the recipe and provide code examples. Compile and run the code. Make adjustments as needed. Repeat this process for each recipe individually. Merge the resulting recipes into the text of the chapter. Add the source code to the **examples** folder.
2. Create pull request

Good luck!
