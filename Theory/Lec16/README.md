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
    + `Debug`, to formant a value using the `{:?}` formatter.


## dyn.rs
+ The Rust compiler needs to know how much space every function's return type requires. This means all your functions have to return a concrete type. 
+ However, there's an easy workaround. Instead of returning a `trait` object directly, our functions return a `Box` which contrain the `trait`.
+ if your function returns a `pointer-to-trait-on-heap` in this way, you need to write the return type with the `dyn` keyword.
    ```rs
    Box<dyn Animal>
    ```

## operator_overloading.rs
+ many of the operators can be overloaded via traits.
+ This is possible because operators are syntactic sugar for method calls. 

## drop.rs
+ The `Drop` trait only has one method: `drop`, which is called automatically when an object goes out of scope.
+ The main use of the `Drop` trait is to free the resources that the implementor instance owns.

## iterator.rs
+ The Iterator trait is used to implement iterators over collections such as arrays.
+ The trait requires only a method to be defined for the next element, which may be manually defined in an impl block or automatically defined (as in arrays and ranges).

## implTraits.rs
+ `impl Trait` can be used in two locations:
    + as an argument type
    + as a return type
+ Note that using `impl Trait` as an argument type means that you cannot explicitly state what form of the function you use
+ If your function returns a type that implements `MyTrait`, you can write its return type as -> impl `MyTrait`
+ More importantly, some Rust types can't be written out. For example, every `closure` has its own unnamed concrete type. 
+ You can also use `impl Trait` to return an iterator that uses `map` or `filter` `closures`! This makes using map and filter easier.

## clone.rs
+ Sometimes we need to make a copy of the resource. The `Clone` trait helps us do exactly this.
