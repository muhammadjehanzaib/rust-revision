# Ownership and Mutability in Rust - Exercises

This document provides a series of exercises aimed at exploring **ownership** and **mutability** in Rust. The exercises help you understand how Rust's ownership model works, as well as the interaction between ownership and mutability.

## Exercises

### Exercise 1: Cloning a String
In this exercise, we create a `String` and use the `clone()` method to create a copy of it. This demonstrates how we can clone data to transfer ownership or retain it in both variables.

```rust
let x = String::from("Hello world");
let y = x.clone();
println!("{}, {}", x, y);
```

# Exercise 2: Taking Ownership
```rust
let s1 = String::from("hello world!");
let s2 = take_ownership(s1);
println!("{}", s2);
```
Explanation: The ownership of s1 is transferred to the function take_ownership. After the function call, s1 is no longer valid, but s2 holds the ownership.

# Exercise 3: Returning Ownership

In this exercise, we define a function give_ownership that creates and returns a String. This shows how ownership can be returned from a function.

```rust
let s = give_ownership();
println!("{}", s);
```
Explanation: The give_ownership function creates a String and returns it. The ownership is transferred back to the calling code when the function returns.

# Exercise 4: Using clone with Ownership

Here, we pass ownership of a String to a function by cloning it, while still retaining the original ownership outside of the function.

```rust
let s = String::from("Hello World");
print_str(s.clone());
println!("{}", s);
```
Explanation: The clone() method creates a new copy of the String that is passed to print_str. This allows us to use s in the calling code after it's passed into the function.

