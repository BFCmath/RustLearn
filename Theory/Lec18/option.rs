// // // // The adult has seen it all, and can handle any drink well.
// // // // All drinks are handled explicitly using `match`.
// // // fn give_adult(drink: Option<&str>) {
// // //     // Specify a course of action for each case.
// // //     match drink {
// // //         Some("lemonade") => println!("Yuck! Too sugary."),
// // //         Some(inner)   => println!("{}? How nice.", inner),
// // //         None          => println!("No drink? Oh well."),
// // //     }
// // // }

// // // // Others will `panic` before drinking sugary drinks.
// // // // All drinks are handled implicitly using `unwrap`.
// // // fn drink(drink: Option<&str>) {
// // //     // `unwrap` returns a `panic` when it receives a `None`.
// // //     let inside = drink.unwrap();
// // //     if inside == "lemonade" { panic!("AAAaaaaa!!!!"); }

// // //     println!("I love {}s!!!!!", inside);
// // // }

// // // fn main() {
// // //     let water  = Some("water");
// // //     let lemonade = Some("lemonade");
// // //     let void  = None;

// // //     give_adult(water);
// // //     give_adult(lemonade);
// // //     give_adult(void);

// // //     let coffee = Some("coffee");
// // //     let nothing = None;

// // //     drink(coffee);
// // //     drink(nothing);
// // // }
// // struct Person {
// //     job: Option<Job>,
// // }

// // #[derive(Clone, Copy)]
// // struct Job {
// //     phone_number: Option<PhoneNumber>,
// // }

// // #[derive(Clone, Copy)]
// // struct PhoneNumber {
// //     area_code: Option<u8>,
// //     number: u32,
// // }

// // impl Person {

// //     // Gets the area code of the phone number of the person's job, if it exists.
// //     fn work_phone_area_code(&self) -> Option<u8> {
// //         // This would need many nested `match` statements without the `?` operator.
// //         // It would take a lot more code - try writing it yourself and see which
// //         // is easier.
// //         self.job?.phone_number?.area_code
// //     }
// // }

// // fn main() {
// //     let p = Person {
// //         job: Some(Job {
// //             phone_number: Some(PhoneNumber {
// //                 area_code: Some(61),
// //                 number: 439222222,
// //             }),
// //         }),
// //     };

// //     assert_eq!(p.work_phone_area_code(), Some(61));
// // }
// #![allow(dead_code)]

// #[derive(Debug)] enum Food { Apple, Carrot, Potato }

// #[derive(Debug)] struct Peeled(Food);
// #[derive(Debug)] struct Chopped(Food);
// #[derive(Debug)] struct Cooked(Food);

// // Peeling food. If there isn't any, then return `None`.
// // Otherwise, return the peeled food.
// fn peel(food: Option<Food>) -> Option<Peeled> {
//     match food {
//         Some(food) => Some(Peeled(food)),
//         None       => None,
//     }
// }

// // Chopping food. If there isn't any, then return `None`.
// // Otherwise, return the chopped food.
// fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
//     // match peeled {
//     //     Some(Peeled(food)) => Some(Chopped(food)),
//     //     None               => None,
//     // }
//     peeled.map(|Peeled(food)| Chopped(food))
// }

// // Cooking food. Here, we showcase `map()` instead of `match` for case handling.
// fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
//     chopped.map(|Chopped(food)| Cooked(food))
// }

// // A function to peel, chop, and cook food all in sequence.
// // We chain multiple uses of `map()` to simplify the code.
// fn process(food: Option<Food>) -> Option<Cooked> {
//     food.map(|f| Peeled(f))
//         .map(|Peeled(f)| Chopped(f))
//         .map(|Chopped(f)| Cooked(f))
// }

// // Check whether there's food or not before trying to eat it!
// fn eat(food: Option<Cooked>) {
//     match food {
//         Some(food) => println!("Mmm. I love {:?}", food),
//         None       => println!("Oh no! It wasn't edible."),
//     }
// }

// fn main() {
//     let apple = None;
//     let carrot = Some(Food::Carrot);
//     let potato = Some(Food::Potato);

//     let cooked_apple = cook(chop(peel(apple)));
//     let cooked_carrot = cook(chop(peel(carrot)));
//     // Let's try the simpler looking `process()` now.
//     let cooked_potato = process(potato);

//     eat(cooked_apple);
//     eat(cooked_carrot);
//     eat(cooked_potato);
// }
#![allow(dead_code)]

#[derive(Debug)] enum Food { CordonBleu, Steak, Sushi }
#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

// We don't have the ingredients to make Sushi.
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _           => Some(food),
    }
}

// We have the recipe for everything except Cordon Bleu.
fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _                => Some(food),
    }
}

// To make a dish, we need both the recipe and the ingredients.
// We can represent the logic with a chain of `match`es:
fn cookable_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None       => None,
        Some(food) => have_ingredients(food),
    }
}

// This can conveniently be rewritten more compactly with `and_then()`:
fn cookable_v3(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

// Otherwise we'd need to `flatten()` an `Option<Option<Food>>`
// to get an `Option<Food>`:
fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).map(have_ingredients).flatten()
}

fn eat(food: Food, day: Day) {
    match cookable_v3(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

fn main() {
    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}