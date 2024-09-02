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
### Unpacking options with ?
+ You can unpack `Options` by using match statements, but it's often easier to use the `?` operator. 
+ If `x` is an `Option`, then evaluating `x?` will return the underlying value if `x` is `Some`, otherwise it will terminate whatever function is being executed and return `None`.
### Mapping
+ Option has a built in method called map(), a combinator for the simple mapping of Some -> Some and None -> None.
+ Multiple map() calls can be chained together for even more flexibility.
+ **Note**:
    + `flat_map` is an iterator adapter method in Rust that allows you to transform each item in an iterator into another iterator, and then flatten the resulting iterators into a single iterator.
### And then
+  Chaining multiple calls together can then become confusing. That's where another combinator called `and_then()`, known in some languages as flatmap, comes in.
+ `and_then()` calls its function input with the wrapped value and returns the result. If the `Option` is `None`, then it returns `None` instead.

## result.rs
+ `Result` is a richer version of the `Option` type that describes possible error instead of possible absence.
+ That is, `Result<T, E>` could have one of two outcomes:
    + `Ok(T)`: An element `T` was found
    + `Err(E)`: An error was found with element `E`
+ By convention, the expected outcome is `Ok` while the unexpected outcome is `Err`.
