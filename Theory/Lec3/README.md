# Lec3 
## Struct.rs
+ struct: nearly the same as C++.
+ Unit struct: These are structs without any fields. Useful when you need to implement a trait on some type but don't have any data to store in the type itself.
+ Tuple struct: These are structs that behave similarly to tuples.
+ Struct update syntax: For all other fields not explicitly set, it copies the values from another.
    + ```let a = Point{x:10.0,y:5.0,z:0.0};```
    + `let b = Point{z:1.0,..a};`
    + `println!("{:?}",b) \\Point { x: 10.0, y: 5.0, z: 1.0 }`

## enum.rs
+ nearly the same as C++
+ match is nearly the same as switch case
    ```rust
    match sth{
        a => do sth,
        b => do sth,
    }
    ```
+ hard-code string is string slice (`&str`). To change type to String, use `.to_owned()` or `String::from()`
+ Type alias: refer to each enum variant via its alias. This might be useful if the enum's name is too long or too generic, and you want to rename it. The most common place you'll see this is in `impl` blocks using the `Self` alias.

## use.rs
+ The `use` declaration can be used so manual scoping isn't needed:
    ```rust
    use crate::Stage::{Beginner, Advanced};
    // Automatically `use` each name inside `Role`.
    use crate::Role::*;
    ```
+ crate: A crate can be thought of as a tree of modules that produces a library or executable. It's similar to a 'package' in some other languages

