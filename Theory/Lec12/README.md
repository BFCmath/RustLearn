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

## Conventions
+  The default binary name is `main`, as we saw before, but you can add additional binaries by placing them in a `bin/` directory:
    ```bash
    foo## Build Scripts
    + TBD...
    
    ├── Cargo.toml
    └── src
        ├── main.rs
        └── bin
            └── my_other_bin.rs
    ```
+ To tell `cargo` to only compile or run this binary, we just pass `cargo` the `--bin my_other_bin flag`, where `my_other_bin` is the name of the binary we want to work with.

## Tesing
+ TBD...

## Build Scripts
+ TBD...
