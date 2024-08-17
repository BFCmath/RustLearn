fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
}

// #![allow(unreachable_code, unused_labels)]

// fn main() {
//     'outer: loop {
//         println!("Entered the outer loop");

//         'inner: loop {
//             println!("Entered the inner loop");

//             // This would break only the inner loop
//             //break;

//             // This breaks the outer loop
//             break 'outer;
//         }

//         println!("This point will never be reached");
//     }

//     println!("Exited the outer loop");
// }


// fn main() {
//     let mut counter = 0;
//     let a = 100;
//     let result = loop {
//         counter += 1;
//         let m = if counter == 5{
//             "a"
//         }
//         else if counter == 10 {
//             "5"
//         }
//         else {
//             "c"
//         };
//         break m
//     };
//     println!("{}",result);
//     // assert_eq!(result, 20);
// }