# macro_rule!
- macros look like functions, except that their name ends with a bang `!`, but instead of generating a function call, macros are expanded into source code that gets compiled with the rest of the program. 
-  Rust macros are expanded into abstract syntax trees, rather than string preprocessing, so you don't get unexpected precedence bugs.
- Macros are created using the `macro_rules!` macro.
```rs
macro_rules! say_hello {
    () => {
        println!("Hello!")
    };
}
```
+ why are macros useful:
    - DRY
    - Domain-specific languages.
    - Variadic interfaces.
## designator.rs
+ The arguments of a macro are prefixed by a dollar sign `$` and type annotated with a **designator**:
    + `block`
    + `expr` is used for expressions
    + `ident` is used for variable/function names
    + `item`
    + `literal` is used for literal constants
    + `pat` (pattern)
    + `path`
    + `stmt` (statement)
    + `tt` (token tree)
    + `ty` (type)
    + `vis` (visibility qualifier)
## overload.rs
+ Macros can be overloaded to accept different combinations of arguments. In that regard, `macro_rules!` can work similarly to a `match` block

## repeat.rs
+ Macros can use `+` in the argument list to indicate that an argument may repeat at least once
+ Macros can use `*` to indicate that the argument may repeat zero or more times.

## DRY 
TBD
## DSL
TBD
## Variadics
TBD




