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
