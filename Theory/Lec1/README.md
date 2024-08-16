# Lec1
## firstprogram.rs
+ `rustc file_name.rs` to compile file.
+ `./file_name` to run. 
+ **Note**: just like C/C++

### Using cargo 
+ Run `cargo new file_name` for auto create rust project. 
+ Run `cargo run` in the created folder to run the program.
+ **Note**: just like npm

## formatedPrint.rs
+ There is something called macro, that end with ! and probably work like function, all printing is handled by macros
+ Learn about macro later (tbd)
+ `std::fmt` contains many traits which govern the display of text. The base form of two important ones are listed below:
    + `mt::Debug`: Uses the `{:?}` marker. Format text for debugging purposes.
    + `fmt::Display`: Uses the  `{}` marker. Format text in a more elegant, user friendly fashion.
+ Implementing the `fmt::Display` trait automatically implements the `ToString` trait which allows us to convert the type to String.

## debugFormat.rs
+ Derive the Debug implementation for customized struct so we can print it by `{:?}` marker.
+ Note that the printable format for debug is "raw" so we need to sacrifice  some elegance.  
+ Uses the `{:#?}` marker for prettier format.

## displayFormat.rs
+ Uses `use` to import module (remember the `;`)
+ Read more about [Tuple struct and Struct](https://stackoverflow.com/questions/30339831/what-are-some-use-cases-for-tuple-structs). *TLDR*: Tuple struct is struct with unnamed fields, while struct is nearly the same in C++.
+ To use the `{}` marker, the trait `fmt::Display` must be implemented. Read [here](https://stackoverflow.com/questions/69477460/is-rust-trait-the-same-as-java-interface) for trait. *TLDR*: now, consider it as interface.
+ Implementing `fmt::Display` for a structure where the elements must each be handled sequentially is tricky. The problem is that each `write!` generates a `fmt::Result`. Proper handling of this requires dealing with all the results. Rust provides the `?` operator for exactly this purpose.
    + `write!(f, "{}", value)?;` (see formatList.rs for better understanding).
+ `Vec`: Seem to be vector in C++, looking for it later (tbd)


