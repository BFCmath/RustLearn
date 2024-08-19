# Lec10
+ Rust provides a powerful module system that can be used to hierarchically split code in logical units (modules), and manage visibility (public/private) between them.
+ A module is a collection of items: functions, structs, traits, `impl` blocks, and even other modules.

## visibility.rs
+ Items in modules default to private visibility.
+ Use the `pub` modifier to override default visibility.
+ Items can access other items in the same module, even when private.
+ Modules can also be nested
+ Functions declared using `pub(in path)` syntax are only visible within the given path. `path` must be a parent or ancestor module
+ Functions declared using `pub(self)` syntax are only visible within the current module, which is the same as leaving them private
+ Functions declared using `pub(super)` syntax are only visible within the parent module
+ `pub(crate)` makes functions visible only within the current crate
+ Nested modules follow the same rules for visibility

## struct_visibility.rs
+ Structs have an extra level of visibility with their fields. 
+ The visibility defaults to private, and can be overridden with the `pub` modifier. 
+ This visibility only matters when a struct is accessed from outside the module where it is defined, and has the goal of hiding information (encapsulation).

+ **Note**:
    + implement a struct with generic type:
        ```rs
        impl<T> ClosedBox<T> {
            
        }
        ``` 

## use 
exactly like `import` in python.

## super and self
exactly like in python.

## file_hierarchy
+ When you declare a module using mod, the compiler looks for the module's contents in several places:
    + Inline, within curly braces after mod declaration.
    + In a file with the same name as the module in the current directory.
    + In a file named mod.rs in a subdirectory named after the module.
        ```rs
        pub mod nested;
        mod inaccessible;
        ```