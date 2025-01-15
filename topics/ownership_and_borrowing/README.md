# Ownership and Mutability in Rust - Exercises
All excercises are covered from rust by practice.

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

# Exercise 5: Destructuring a Tuple
In this exercise, we demonstrate ownership and immutability with tuples. A tuple is moved when assigned to another variable, and its contents are also moved if they implement the Copy trait.

```rust
let x: (i32, i32, (), &str) = (1, 2, (), "hello");
let y = x;
println!("{:?} , {:?}", x, y);
```
Explanation: Tuples can store values of different types, and when a tuple is moved, ownership of its contents is also transferred. The code will not compile because x is moved to y and cannot be accessed after that.

# Exercise 6: Mutability with Ownership Transfer

In this exercise, we create a mutable variable and push new data into it. It demonstrates how mutability works when ownership is transferred.

```rust
let s = String::from("hello");
let mut s1 = s;

s1.push_str("world");
println!("Success!");
```
Explanation: The ownership of s is transferred to s1. Since s1 is mutable, we can modify it. Rust allows the modification of the data after transferring ownership to a mutable variable.

# Function

```rust
#[allow(dead_code)]
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}
```
Explanation: This function takes ownership of the String and returns it, demonstrating how ownership can be passed around.

# print_str
```rust
#[allow(dead_code)]
fn print_str(s: String) {
    println!("{}", s);
}
```
Explanation: This function takes ownership of a String and prints it. Once the function ends, the ownership of s is dropped.

# give_ownership

```rust 
#[allow(dead_code)]
fn give_ownership() -> String {
    let s = String::from("Hello world");
    let _s = s.as_bytes(); // Convert String to Vec
    s
}
```
Explanation: This function returns a String that demonstrates ownership being returned from a function.

# Exercise 7: Using Box with Ownership

In this exercise, we create a Box<i32> to demonstrate ownership and mutability with heap-allocated data. We mutate the value inside the box, then assert its value and print a success message.

```rust
let x: Box<i32> = Box::new(5);
let mut y: Box<i32> = Box::new(1);

*y = 4;

assert_eq!(*x, 5);
println!("Success");
```
Explanation: Here, x is a Box<i32> that points to a value on the heap. We mutate the value inside y but do not alter x. The assertion confirms that the value of x remains unchanged after modifying y. This demonstrates how mutability works with heap-allocated data inside a Box.

# Exercise 8: Tuple Ownership with Move
In this exercise, we move the first element of a tuple into a new variable. This demonstrates how ownership is transferred when destructuring a tuple.

```rust
let t: (String, String) = (String::from("hello"), String::from("World"));

let _s: String = t.0;

println!("{}", t.1);
```
Explanation: When the first element (t.0) is moved into _s, it is no longer available in the original tuple t. The second element t.1 can still be accessed.

# Exercise 9: Cloning a Tuple

Here, we use clone to clone a tuple, and destructure it into two new variables. This demonstrates the use of cloning with complex types like tuples.

```rust
let t: (String, String) = (String::from("hello"), String::from("world"));
let (s1, s2) = t.clone();
println!("{:?}, {:?} , {:?}", s1, s2, t);
```
Explanation: By cloning the tuple t, we create a copy of the original tuple's values. The cloned values are then destructured into s1 and s2. The original tuple t remains intact and is printed after the clone.
