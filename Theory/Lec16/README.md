# Lec16

## traits.rs
+ A `trait` is a collection of methods defined for an unknown type: `Self`. They can access other methods declared in the same trait.
+ Traits can be implemented for any data type.
+ nearly like `interface`

## derive.rs
+ The compiler is capable of providing basic implementations for some traits via the `#[derive]` attribute. These traits can still be manually implemented if a more complex behavior is required.
    + Comparison traits: `Eq, PartialEq, Ord, PartialOrd`.
    + `Clone`, to create `T` from `&T` via a copy.
    + `Copy`, to give a type 'copy semantics' instead of 'move semantics'.
    + `Hash`, to compute a hash from `&T`.
    + `Default`, to create an empty instance of a data type.
    + `Debug`, to format a value using the `{:?}` formatter.


## dyn.rs
+ The Rust compiler needs to know how much space every function's return type requires. This means all your functions have to return a concrete type. 
+ However, there's an easy workaround. Instead of returning a `trait` object directly, our functions return a `Box` which contrain the `trait`.
+ if your function returns a `pointer-to-trait-on-heap` in this way, you need to write the return type with the `dyn` keyword.
    ```rs
    Box<dyn Animal>
    ```