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

