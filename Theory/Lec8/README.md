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

## while.rs
- Same as other languages and `loop`

## for_range.rs
- The `for` `in` construct can be used to iterate through an `Iterator`. 
- One of the easiest ways to create an iterator is to use the range notation `a..b`. This yields values from `a` (inclusive) to `b` (exclusive) in steps of one.
- `a..=b` can be used for a range that is inclusive on both ends.
### iterators
+ `into_iter`, `iter` and `iter_mut` all handle the conversion of a collection into an iterator in different ways, by providing different views on the data within.
    + `iter` - This borrows each element of the collection through each iteration. Thus leaving the collection untouched and available for reuse after the loop.
    + `into_iter` - This consumes the collection so that on each iteration the exact data is provided. Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.
    + `iter_mut` - This mutably borrows each element of the collection, allowing for the collection to be modified in place.
+ **Note**: 
    + `iter_mut()` provides `&mut T` instead of `T`, so we do need to add `&mut` when compare.
    + `&mut` is a mutable reference, so we do need to dereference  before assigning.
        ```rust
        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!", 
                _ => "Hello",
            }
        }
        ```

## match.rs
+ like C++ switch, but auto break at the end and can stack:
    + `2 | 3 | 5 | 7 | 11`
    + `13..=19`
    + `_`: default
+ `match` is an expression. 
## destructing.rs
+ `..` can be used to ignore the rest of the tuple
+ Single values can be ignored with `_`
+ `name@..` captures all remaining elements (if any) into a slice named `name`
+ `ref`: get reference/pointer of a var 
    ```rs
    let ref _is_a_reference = 3;
    ```
+ You do not need a match block to destructure structs:
    ```rs
    let faa = Foo { x: (1, 2), y: 3 };
    let Foo { x : x0, y: y0 } = faa;
    ```
**note**: format print a pointer normally will print its value instead of itself. In order to print pointer, use `{:p}`

## guard.rs 
+ A `match` *guard* can be added to filter the arm.
+ Note that the compiler won't take guard conditions into account when checking if all patterns are covered by the match expression.

## binding.rs
+ `match` provides the `@` sigil for binding values to names.

## iflet.rs 
+ For some use cases, when matching enums, `match` is awkward. `if let` is cleaner for this use case and in addition allows various failure options to be specified.
+ Another benefit is that if let allows us to match non-parameterized enum variants. 
    + This is true even in cases where the enum doesn't implement or derive PartialEq. 
    + In such cases if Foo::Bar == a would fail to compile, because instances of the enum cannot be equated, however if let will continue to work.

## letelse.rs
+ With `let-else`, a refutable pattern can match and bind variables in the surrounding scope like a normal `let`, or `else` diverge (e.g. `break`, `return`, `panic!`) when the pattern doesn't match.

## whilelet.rs
+ Similar to `if let`, `while let` can make awkward match sequences more tolerable

