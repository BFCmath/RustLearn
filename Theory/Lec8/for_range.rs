// fn main() {
//     // A counter variable
//     let mut n = 1;

//     // Loop while `n` is less than 101
//     while n < 101 {
//         if n % 15 == 0 {
//             println!("fizzbuzz");
//         } else if n % 3 == 0 {
//             println!("fizz");
//         } else if n % 5 == 0 {
//             println!("buzz");
//         } else {
//             println!("{}", n);
//         }

//         // Increment counter
//         n += 1;
//     }
// }

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];
    
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!", //note that the name is &mut &str so we need
                                                             //to add &mut before "Ferris" 
                                                             // by default, "Ferris" is &str
            _ => "Hello",
        }
    }
    print_type_of(&a);
    println!("names: {:?}", names);
}