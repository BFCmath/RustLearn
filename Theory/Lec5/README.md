# Lec5
## casting.rs
+ Rust provides no implicit type conversion (coercion) between primitive types. But, explicit type conversion (casting) can be performed using the as keyword. (so not like in C++)
+ Rules for converting between integral types follow C conventions generally, except in cases where C has undefined behavior. The behavior of all casts between integral types is well defined in Rust. (so that means no vague error)
+ the `as` keyword performs a *saturating cast* when casting from float to int. If the floating point value exceeds the upper bound or is less than the lower bound, the returned value will be equal to the bound crossed.

**Note**:
- literals are like in C++: `2u32` or more clearly with underscore ` 3_f32`
- crate(package/library) -> module -> function 