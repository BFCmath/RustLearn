# Lec3 
## Struct.rs
+ struct: nearly the same as C++.
+ Unit struct: These are structs without any fields. Useful when you need to implement a trait on some type but don't have any data to store in the type itself.
+ Tuple struct: These are structs that behave similarly to tuples.
+ Struct update syntax: For all other fields not explicitly set, it copies the values from another.
    + ```let a = Point{x:10.0,y:5.0,z:0.0};```
    + `let b = Point{z:1.0,..a};`
    + `println!("{:?}",b) \\Point { x: 10.0, y: 5.0, z: 1.0 }`


