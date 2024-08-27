# Error handling
## panic.rs
+ The simplest error handling mechanism we will see is panic. 
+ It prints an error message, starts unwinding the stack, and usually exits the program. 

## option.rs
+ An enum called `Option<T>` in the std library is used when absence is a possibility. It manifests itself as one of two "options":
    + `Some(T)`: An element of type T was found
    + `None`: No element was found
+ These cases can either be explicitly handled via `match` or implicitly with `unwrap`. 
+ Implicit handling will either return the inner element or panic.