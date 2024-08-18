# Lec9
## function.rs
- Functions are declared using the fn keyword. 
- Arguments and return just like Python
- The final expression in the function will be used as return value. 
- Alternatively, the return statement can be used, just like in C++.

## methods.rs
+ Some functions are connected to a particular type. 
+ These come in two forms: 
    - Associated functions
    - Methods
+ Associated functions are functions that are defined on a type generally, while methods are associated functions that are called on a particular instance of a type.

### Associated function
+ Associated functions are functions associated with particular struct.
+ No need to init an instance to use associated functions.
    ```rs
    Point::origin()
    ```
+ Associated functions are generally used like constructors.
### Method
+ Sugar (Syntactic Sugar): Syntactic sugar refers to syntax that makes code easier to read or express.
    + &self is sugar for self: &Self
    + &mut self is sugar for self: &mut Self

**Note**:
+ Ownership: 
    + Every value has a single owner, which is the variable that owns the data. 
    + Ownership rules ensure that there is only one owner of a value at a time, and when the owner goes out of scope, the value is automatically cleaned up. 
    + When you assign one variable to another, or pass a variable as a parameter to a function, ownership is transferred, or "moved" from one variable to another. After a move, the original variable is no longer usable.
+ Borrowing:
    + Temporarily allow another variable to access the data without transferring ownership. Rust achieves this through references.
    + You can have either one mutable reference or any number of immutable references, but not both at the same time.
    + References must always be valid (no dangling references).


## closure.rs
+ Closures are functions that can capture the enclosing environment.
+ Other characteristics of closures include:
    + using `||` instead of `()` around input variables.
    + optional body delimitation (`{}`) for a single line expression (mandatory otherwise).
    + the ability to capture the outer environment variables.

## capture.rs
+ Closures are inherently flexible and will do what the functionality requires to make the closure work without annotation.
+ Closures can capture variables:
    - by reference: `&T`
    - by mutable reference: `&mut T`
    - by value: `T`
+ They preferentially capture variables by reference and only go lower when required.
+ The closure remains borrow until the last used of that closure.
+ The type of closure also needs to change according to the type of used outer vars.
+ A copy type would copy into the closure leaving the original untouched. 
+ A non-copy must move immediately into the closure.
+ Using `move` before vertical pipes forces `closure` to take ownership of captured variables.
+ **Note**:
    + The i32 type is a simple scalar type that implements the Copy trait. Types that implement the Copy trait have a special behavior when assigned or passed to a function: instead of moving ownership, a bitwise copy of the value is made.
        ```rs
        let mut count = 0;
        let _count_reborrowed = count; 
        println!("{}",count);
        count +=1;
        ```
    + `Box`: A non-copy type.  
    + `mem::drop` requires `T` so this must take by value. 
    + `Vec`: has non-copy semantics.
