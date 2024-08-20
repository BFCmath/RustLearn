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
## traits.rs
+ `trait`s can also be generic.
    ```rs
    impl<T, U> DoubleDrop<T> for U {
        fn double_drop(self, _: T) {}
    }
    ```

## bounds.rs
- When working with generics, the type parameters often must use traits as **bounds** to stipulate what functionality a type implements. 
- A consequence of how bounds work is that even if a trait doesn't include any functionality, you can still use it as a bound. 
- **Note**: this can replace inheritance

## multibounds.rs
- Multiple bounds for a single type can be applied with a `+`.
- Like normal, different types are separated with `,`.

## where.rs
- A bound can also be expressed using a where clause immediately before the opening `{`, rather than at the type's first mention. 
- Additionally, where clauses can apply bounds to arbitrary types, rather than just to type parameters.
    ```rs
    impl<T> PrintInOption for T where
        Option<T>: Debug {
        fn print_in_option(self) {
            println!("{:?}", Some(self));
        }
    }
    ```

+ **Note**: 
    + `Reference Transparency`: In Rust, when you have a reference to a type, you can access its fields and methods as if you were working with the type directly.
        ```rs
        fn is_adult(age: &Years) -> bool {
            age.0 >= 18
        }
        ```
## associated_item.rs
+ The use of "Associated types" improves the overall readability of code by moving inner types locally into a trait as output types.
    ```rs
    // Without using associated types
    fn difference<A, B, C>(container: &C) -> i32 where
        C: Contains<A, B> { ... }
    // Using associated types
    fn difference<C: Contains>(container: &C) -> i32 { ... }
    ```
    ```rs
    trait Contains {
        // Define generic types here which methods will be able to utilize.
        type A;
        type B;
    }
    
    impl Contains for Container {
        type A = i32;
        type B = i32;
    }
    ```
+ **Note**: `type` in this context is different from `type` when used for aliases

## Phantom Type 
+ TBD...