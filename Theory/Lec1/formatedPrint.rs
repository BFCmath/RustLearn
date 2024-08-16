fn main(){
    println!("Hello, World!"); 
    // println! print the string with a newline at the end
    print!("Hello, World!\n");
    // print! print the string without a newline at the end
    eprint!("Hello, World!\n");
    // eprint! print the string to the standard error without a newline at the end
    eprintln!("Hello, World!");
    // eprintln! print the string to the standard error with a newline at the end
    
    // format! is used to format the string
    let s = format!("Hello, World!");
    println!("{}", s);

    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="the quick brown fox",
            verb="jumps over");

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number=1); // 00001
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number=1); // 10000

    // Rust even checks to make sure the correct number of arguments are used.
    // println!("My name is {0}, {1} {0}", "Bond");
    // Only available for rust-analyzer

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}