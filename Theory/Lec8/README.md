# Lec8
## if.rs
- Branching with `if-else` is similar to other languages. 
- Unlike many of them, the boolean condition doesn't need to be surrounded by parentheses, and each condition is followed by a block. 
- `if-else` conditionals are expressions, and, all branches must return the same type.
    ```rs
    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            10 * n
        } else {
            println!(", and is a big number, halve the number");
            n / 2 // expression return the same type
        }; // semicolon
    ```
## loop.rs
- Rust provides a `loop` keyword to indicate an infinite loop.
- `break` and `continue` are used like other languages.
- Label a `loop` with `'label` can help to break or continue outer loops when dealing with nested loops.
- **Note**: `'static` is not a `label`
- Return from loops: put value after the break, and it will be returned by the loop expression.
- **Note**: 
    + You need to return the same type.
    + `break;` will return `()` just like `if`
    + You dont need to have a semicolon at the end of break.
    + You can also use `'label`.
        ```
        break 'outer 5;
        ```