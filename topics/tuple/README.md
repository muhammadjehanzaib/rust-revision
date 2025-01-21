# Rust Tuples Exercises

This document explains several exercises demonstrating the use of tuples in Rust. Tuples are fixed-size collections of values of potentially different types.

## Exercise 1: Tuple Types

```rust
let _t0: (u8, i16) = (2, -1);
// Tuples can have tuple's members
let _t1: (u8, (i16, i32)) = (3,(-3, 44));
let _t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
println!("Success!");
```
This exercise introduces the syntax for defining tuples and their types.

_t0: (u8, i16): A tuple with an unsigned 8-bit integer and a signed 16-bit integer.
_t1: (u8, (i16, i32)): A tuple containing an unsigned 8-bit integer and another nested tuple of a signed 16-bit integer and a signed 32-bit integer.
_t: (u8, u16, i64, &str, String): A tuple with various types, including unsigned integers, a signed 64-bit integer, a string slice (&str), and a String.

# Exercise 2: Tuple Indexing

```rust 
let t = ("i", "am", "sunface");
assert_eq!(t.2, "sunface");
println!("Success");
```
This demonstrates how to access elements of a tuple using indexing. Tuple indices start at 0. t.2 accesses the third element of the tuple t.

# Exercise 3: Printing Long Tuples

```rust
let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
println!("too long tuple {:#?}", too_long_tuple);
```
This exercise highlights a limitation: tuples with 12 or more elements cannot be directly printed using the standard println! macro with the {:?} or {:#?} format specifiers. If you try to compile this code, you'll get a compiler error. This is a current limitation of Rust's standard library.

# Exercise 4: Tuple Destructuring

```rust
let tup = (1, 6.4, "hello");
let (x, z, y) = tup;

assert_eq!(x, 1);
assert_eq!(y, "hello");
assert_eq!(z, 6.4);

println!("Success!");
```

This shows how to destructure a tuple using a pattern. The values of the tuple are assigned to the variables x, z, and y in the order they appear in the tuple.

# Exercise 5: Destructuring with Declaration
```rust
let (x, y, z);
(x, y, z) = (3, 1, 2);

assert_eq!(x, 3);
assert_eq!(y, 1);
assert_eq!(z, 2);
println!("Success");
```
This demonstrates that you can declare the variables first and then assign values to them from a tuple using destructuring.


# Exercise 6: Tuples as Function Arguments and Return Values
```rust 
fn main() {
    // 🌟🌟 Tuples can be used as function arguments and return values
    let (x, y) = sum_multiply((2, 3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);
    println!("Success");
}

fn sum_multiply(num: (i32, i32)) -> (i32, i32) {
    (num.0 + num.1, num.0 * num.1)
}
```
This exercise demonstrates a common use case for tuples: returning multiple values from a function. The sum_multiply function takes a tuple of two i32 values and returns a tuple containing their sum and product. This is a clean way to return related data from a function.

