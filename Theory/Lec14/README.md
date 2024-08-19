# Lec14
## generics.rs
+ Generics is the topic of generalizing types and functionalities to broader cases.
+ "Generic type parameters" are typically represented as `<T>`
    ```rs
    fn foo<T>(arg: T) { ... }
    ```
## function.rs
+ Using generic functions sometimes requires explicitly specifying type parameters. This may be the case if the function is called where the return type is generic, or if the compiler doesn't have enough information to infer the necessary type parameters.
+ A function call with explicitly specified type parameters looks like: ```fun::<A, B, ...>()```.

## implementation.rs
+ Similar to functions, implementations require care to remain generic.
    ```rs
    impl<T> GenericVal<T> {}
    ```
