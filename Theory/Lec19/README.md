# Lec19
## box.rs
+ All values in Rust are stack allocated by default.
+ Values can be boxed (allocated on the heap) by creating a `Box<T>`.
+ A box is a smart pointer to a heap allocated value of type `T`.
+ When a box goes out of scope, its destructor is called, the inner object is destroyed, and the memory on the heap is freed.
+ Boxed values can be dereferenced using the `*` operator; this removes one layer of indirection.

## vector.rs
+ Vectors are re-sizable arrays.
+ Like slices, their size is not known at compile time, but they can grow or shrink at any time. 
+ The `vec!` macro can be used to initialize a vector
+ The `len` method yields the number of elements currently stored in a vector
+ 
+ **Note**:
    + The `collect()` method is used to transform an iterator into a collection.

## string.rs
+ A `String` is stored as a vector of bytes (`Vec<u8>`), but guaranteed to always be a valid UTF-8 sequence.
+ `String` is heap allocated, growable and not null terminated.
    ```rs
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    let alice = String::from("I like dogs");
    ```
## hashmap.rs
+ `HashMap` keys can be booleans, integers, strings, or any other type that implements the `Eq` and `Hash` traits. More on this in the next section.
+ Takes a reference and returns `Option<&V>`
+ `HashMap::iter()` returns an iterator that yields `(&'a key, &'a value)` pairs in arbitrary order.
+ Any type that implements the `Eq` and `Hash` traits can be a key in `HashMap`

## hashset.rs
+ A `HashSet`'s unique feature is that it is guaranteed to not have duplicate elements.
+ If you insert a value that is already present in the `HashSet`, (i.e. the new value is equal to the existing and they both have the same hash), then the new value will replace the old.

## rc.rs
+ When multiple ownership is needed, `Rc`(Reference Counting) can be used.
+ `Rc` keeps track of the number of the references which means the number of owners of the value wrapped inside an `Rc`.
+ Cloning an `Rc` never performs a deep copy. Cloning creates just another pointer to the wrapped value, and increments the count.

## Arc
+ When shared ownership between threads is needed, `Arc`(Atomically Reference Counted) can be used.
+ This struct, via the `Clone` implementation can create a reference pointer for the location of a value in the memory heap while increasing the reference counter. 
+ As it shares ownership between threads, when the last reference pointer to a value is out of scope, the variable is dropped.