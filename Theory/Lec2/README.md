# Lec2
## Premitives.rs
+ Scalar 
    + Signed int: `i8`, `i16`, `i32`, `i64`, `i128` and `isize`
    + Unsigned int: `u8`, `u16`, `u32`, `u64`, `u128` and `usize`
    + Float: `f32`, `f64`
    + `char`
    + `bool`
    + The unit type `()`, whose only possible value is an empty tuple: `()`
+ Compound
    + Arrays like [1, 2, 3]
    + Tuples like (1, true)

+ A mutable variable's value can be changed: `mut`
+ The type of a variable can't be changed.
+ Variables can be overwritten with shadowing.

## Literal_Operator.rs
+ Nearly the same as python. 

## Tuple
+ Nearly the same as list numpy
+ Can debug print when < 12 elements
+ Access elements by .0 .1 insteads of [0], [1] like others
+ Can destructure like in Python
+ To create one element tuples, the comma is required to tell them apart from a literal surrounded by parentheses. 
    + `println!("One element tuple: {:?}", (5u32,)); \\ (5,)`
    + `println!("Just an integer: {:?}", (5u32)); \\ 5`
+ function seems to look like ipynb, note that do not need `return`

