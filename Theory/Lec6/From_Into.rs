use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

// Implement `From` trait for `Number`
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str); // convert &str to String
    let num = Number::from(30); // convert i32 to Number
    println!("My number is {:?}", num); 
}


// use std::convert::Into;

// #[derive(Debug)]
// struct Number {
//     value: i32,
// }

// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }

// fn main() {
//     let int = 5;
//     // Try removing the type annotation
//     let num: Number = int.into();
//     println!("My number is {:?}", num);
// }

// use std::convert::From;

// #[derive(Debug)]
// struct Number {
//     value: i32,
// }

// // Define `From`
// impl From<i32> for Number {
//     fn from(item: i32) -> Self {
//         Number { value: item }
//     }
// }

// fn main() {
//     let int = 5;
//     // use `Into`
//     let num: Number = int.into();
//     println!("My number is {:?}", num);