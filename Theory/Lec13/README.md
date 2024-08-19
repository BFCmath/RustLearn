# Lec13
+ An `attribute` is metadata applied to some module, crate or item. This metadata can be used to/for:
    + conditional compilation of code
    + set crate name, version and type (binary or library)
    + disable lints (warnings)
    + enable compiler features (macros, glob imports, etc.)
    + link to a foreign library
    + mark functions as unit tests
    + mark functions that will be part of a benchmark
    + attribute like macros
+ Attributes look like `#[outer_attribute]` or `#![inner_attribute]`, with the difference between them being where they apply.
    + `#[outer_attribute]` applies to the item immediately following it.
    + `#![inner_attribute]`  applies to the enclosing item (typically a module or a crate). 
+ Attributes can take arguments with different syntaxes.
+ Attributes can have multiple values and can be separated over multiple lines, too.

## dead_code.rs
+ The compiler provides a dead_code lint that will warn about unused functions. An attribute can be used to disable the lint.

## crate_type.rs
+ The crate_type attribute can be used to tell the compiler whether a crate is a binary or a library
+ The crate_name attribute can be used to set the name of the crate.
+ Both the crate_type and crate_name attributes have no effect whatsoever when using Cargo, the Rust package manager. 

## cfg.rs
+ Configuration conditional checks are possible through two different operators:
    + the `cfg` attribute: `#[cfg(...)]` in attribute position
    + the `cfg!` macro: `cfg!(...)` in boolean expressions

+ While the former enables conditional compilation, the latter conditionally evaluates to `true` or `false` literals allowing for checks at run-time. Both utilize identical argument syntax.
+ Some conditionals like `target_os` are implicitly provided by `rustc`, but custom conditionals must be passed to `rustc` using the `--cfg` flag.
    ```rs
    #[cfg(some_condition)]
    fn conditional_function() {
        println!("condition met!");
    }

    fn main() {
        conditional_function();
    }
    ```
    ```bash
    $ rustc --cfg some_condition custom.rs && ./custom
    condition met!
    ```