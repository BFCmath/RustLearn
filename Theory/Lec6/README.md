# Lec6
## From_Into.rs
- The `From` and `Into` traits are inherently linked, and this is actually part of its implementation. If you are able to convert type A from type B, then it should be easy to believe that we should be able to convert type B to type A.
- A type can have many `impl From/Into`s. in this case, we need to give the return value an explicit type. 
- `From` and `Into` are interchangable. If you have implemented the `From` trait for your type, `Into` will call it when necessary. Note, however, that the converse is not true: implementing `Into` for your type will not automatically provide it with an implementation of `From`.
- About `self/Self`:
    + `self` is just like in C++.
    + `&self` is used when you want to borrow the instance immutably.\
    + `&mut self` is used when you need to borrow the instance mutably to modify it.
    + `Self`  is used as a type alias for the implementing type.

## TryFrom_TryIntro.rs
- The `TryFrom`/`TryInto` traits are used for fallible conversions, and as such, return `Results`.
- The `Result` type is used for returning and propagating errors. It has two variants: 
    ```rust
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    ```
- The `Option` type is used when a value could be something or nothing. It has two variants:
    ```rust
    enum Option<T> {
        Some(T),
        None,
    }
    ```
- `PartialEq` allows the struct to be compared with `==` and `!=`.
- `assert_eq!`: This is a macro used for testing equality in Rust.

## To_FromString.rs
- To convert any type to a `String` is as simple as implementing the `ToString` trait for the type. Rather than doing so directly, you should implement the `fmt::Display` trait which automagically provides `ToString` and also allows printing the type as discussed [here](Theory/Lec1/README.md).
- It's useful to convert strings into many types, but one of the more common string operations is to convert them from string to number. 
    ```rust
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    ```
- **Note**: `unwrap` right here is used for `Ok(T)` which is the return value of `parse()`. `unwrap` will return `T`.