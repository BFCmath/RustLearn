# Lec4
## variable_binding.rs
+ Values (like literals) can be bound to variables, using the `let` binding.
+ Variable bindings are immutable by default, but this can be overridden using the `mut` modifier.
+ Scope and Shadowing: Scope: just like in C++, Shadowing: just like python
+ Declare first: It's possible to declare variable bindings first, and initialize them later. However, this form is seldom used, as it may lead to the use of uninitialized variables.
+ Freezing: When data is bound by the same name immutably, it also freezes. Frozen data can't be modified until the immutable binding goes out of scope. 

