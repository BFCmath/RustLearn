# Lec15
+ Scopes play an important part in ownership, borrowing, and lifetimes. 
+ they indicate to the compiler when borrows are valid, when resources can be freed, and when variables are created or destroyed.


## RAll
+ Variables in Rust do more than just hold data in the stack: they also own resources, e.g. `Box<T>` owns memory in the heap. 
+ Rust enforces `RAII` (Resource Acquisition Is Initialization), so whenever an object goes out of scope, its destructor is called and its owned resources are freed.

## Ownership and moves
+ Because variables are in charge of freeing their own resources, resources can only have one owner. 
+ When doing assignments (`let x = y`) or passing function arguments by value (`foo(x)`), the ownership of the resources is transferred. In Rust-speak, this is known as a move.
+ After moving resources, the previous owner can no longer be used. This avoids creating dangling pointers.

+ Mutability of data can be changed when ownership is transferred.
+ **Note**: 
    + `Box<T>` is a **smart pointer** that owns a heap allocation of type `T`.
    + To access or modify the value inside a `Box`, we need to dereference it using `*`.
    + `Automatic dereferencing` is a feature of Rust that makes working with smart pointers like `Box` more convenient. It allows you to treat a `Box<T>` as if it were a `T` in many contexts.

+ Within the destructuring of a single variable, both `by-move` and `by-reference` pattern bindings can be used at the same time.
+ Doing this will result in a *partial move* of the variable, which means that parts of the variable will be moved while other parts stay.
+ In such a case, the parent variable cannot be used afterwards as a whole, however the parts that are only referenced (and not moved) can still be used.

## Borrow
+ Instead of passing objects by value (`T`), objects can be passed by reference (`&T`).
+ Mutable data can be mutably borrowed using `&mut T`. This is called a mutable reference and gives read/write access to the borrower.
+ **Note**:
    +  the `_` pattern is special in Rust:
        + `_` is a wildcard pattern that matches any value.
        + Importantly, it does not bind the value to a variable.
        + Because it doesn't bind the value, it also doesn't move or copy the value.
    + Mutability is not inherited through ownership transfer.
    + To make `y` mutable when taking ownership, you need to explicitly declare it as mutable: `let mut y = x`;
    + You can have a mutable reference to a mutable variable, but not to an immutable variable.
    + You can have a immutable variable reference to a mutable variable.
## Lifetimes
+ A lifetime is a construct the compiler (or more specifically, its borrow checker) uses to ensure all borrows are valid.
+ Specifically, a variable's lifetime begins when it is created and ends when it is destroyed. 
+ While lifetimes and scopes are often referred to together, they are not the same.
### Explicit annotation
+ The borrow checker uses explicit lifetime annotations to determine how long references should be valid.
+ In cases where lifetimes are not elided1, Rust requires explicit annotations to determine what the lifetime of a reference should be. 
+ TBD...