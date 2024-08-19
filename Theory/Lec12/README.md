# Lec12

+ `cargo` is the official Rust package management tool. 
+ It has lots of really useful features to improve code quality and developer velocity!
+ These include:
    + Dependency management and integration with crates.io (the official Rust package registry)
    + Awareness of unit tests
    + Awareness of benchmarks

## Dependencies
+ Create a new Rust project
    ```bash
    # A binary
    cargo new foo
    # A library
    cargo new --lib bar
    ```
+ The file hierarchy after that:
    ```bash
    .
    ├── bar
    │   ├── Cargo.toml
    │   └── src
    │       └── lib.rs
    └── foo
        ├── Cargo.toml
        └── src
            └── main.rs
    ```

+ `Cargo.toml`
    ```toml
    [package]
    name = "foo"
    version = "0.1.0"
    authors = ["mark"]

    [dependencies]
    ```

+ The `[dependencies]` section lets you add dependencies for your project.
    ```toml
    [dependencies]
    clap = "2.27.1" # from crates.io
    rand = { git = "https://github.com/rust-lang-nursery/rand" } # from online repo
    bar = { path = "../bar" } # from a path in the local filesystem
    ```
+ To build our project we can execute `cargo build` anywhere in the project directory (including subdirectories!)
+ We can also do `cargo run` to build and run.