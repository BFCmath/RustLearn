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