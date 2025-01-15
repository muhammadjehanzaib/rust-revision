# References and Borrowing in Rust - Exercises
All excercises are covered from rust by practice.

This document provides a series of exercises aimed at exploring **references** and **borrowing** in Rust. The exercises help you understand how Rust handles references and borrowing, ensuring memory safety while allowing for efficient data access.

## Exercises

### Exercise 1: Creating a Reference
In this exercise, we create a reference to an integer and print the memory address of the reference.

```rust
let x = 5;
// Fill the blank
let p = &x;
println!("{:p}", p);
```
Explanation: The &x creates a reference to the integer x. The println!("{:p}", p) prints the memory address of the reference.

# Exercise 2: Dereferencing a Reference
In this exercise, we dereference a reference to access the value it points to.

```rust
let x = 5;
let y = &x; // reference to y

assert_eq!(5, *y); // dereference to y
println!("Success");
```
Explanation: y is a reference to x, and *y dereferences y to access the value 5.

# Exercise 3: Borrowing an Object (Immutable)
In this exercise, we borrow an immutable reference to a String and pass it to a function.

```rust
let mut s = String::from("hello, ");

borrow_object(&s);
println!("Success");
```
Explanation: The &s creates an immutable reference to s that is passed to the function borrow_object. The borrow_object function does not modify s.


# Exercise 4: Borrowing an Object (Mutable)
In this exercise, we borrow a mutable reference to a String and modify it within a function.

```rust
let mut s = String::from("hello, ");
push_str(&mut s);
println!("Success");
```
Explanation: The &mut s creates a mutable reference to s. The push_str function modifies the string by appending "world" to it.


# Exercise 5: Using a Mutable Reference
In this exercise, we use a mutable reference to modify a String directly.

```rust
let mut s = String::from("hello, ");
let p = &mut s;
p.push_str("world");
println!("Success");
```
Explanation: The &mut s creates a mutable reference p to s. The push_str method modifies the string by appending "world" to it.


# Exercise 6: Using ref for Borrowing
In this exercise, we use ref to borrow a value similar to using &.

```rust

let x = 'V';
let r1 = &x;
let ref r2 = x;

assert_eq!(*r1, *r2);
assert_eq!(get_addr(r1), get_addr(r2));
println!("Success");
```
Explanation: ref is used to take a reference to x. Both r1 and r2 refer to the same value, and we can check that they point to the same memory address.


# Exercise 7: Borrowing Rules (Mutable Reference)
In this exercise, we try to create a mutable reference to a String and borrow it.

```rust
let mut s = String::from("hello");
let r1 = &mut s;

println!("{}", r1);
println!("Success");
```
Explanation: The mutable reference r1 is valid and can be used to modify s. However, no other references to s can coexist while r1 exists.

# Exercise 8: Borrowing an Object (Mutable)

In this exercise, we borrow a mutable reference to a String and use it in a function.

```rust
let mut s = String::from("hello, ");
borrow_object(&mut s);
println!("Success");
```

Explanation: The &mut s creates a mutable reference to s. This mutable reference is passed to the borrow_object function.


# Exercise 9: Borrowing an Object (Immutable and Mutable)

In this exercise, we attempt to borrow an immutable and mutable reference to the same object.

```rust
let mut s = String::from("hello ,");
borrow_object(&s);
s.push_str("world");
println!("Success");
```
Explanation: The code demonstrates borrowing an immutable reference to s using &s and then modifying s with s.push_str("world"). The Rust borrow checker ensures that mutable references are not created while immutable ones exist.

# Exercise 10: Mutable Borrowing
In this exercise, we create a mutable reference and attempt to create a second one (which would cause a compiler error).

```rust
let mut s = String::from("hello, ");

let r1 = &mut s;
r1.push_str("world");
// let r2 = &mut s; // This will cause an error
// r2.push_str("!");

println!("{}", r1);
```
Explanation: The second mutable reference r2 would cause a borrow conflict. Rust does not allow multiple mutable references to the same object at the same time, ensuring no data races.

# Exercise 11: Multiple Mutable References
In this exercise, we attempt to create two mutable references to the same object, which will cause a compiler error.

```rust
let mut s = String::from("hello, ");
let r1 = &mut s;
let r2 = &mut s;

r1.push_str("string");
// Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
// You can't use r1 and r2 at the same time
```
Explanation: The code will not compile because you cannot have two mutable references (r1 and r2) to s simultaneously in Rust.
