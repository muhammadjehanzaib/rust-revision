# Rust String Exercises

This document contains a series of exercises designed to explore the intricacies of strings in Rust, including `String` and `&str`, ownership, UTF-8 encoding, and capacity.

## Exercise 1: Basic String Operations

```rust
fn main() {
    let mut s: String = String::from("Hello");

    s.push_str(", world");
    s.push('!');

    move_ownership(s.clone()); // Clone to avoid ownership transfer

    assert_eq!(s, "Hello, world!");

    println!("Success");
}

fn move_ownership(s: String) {
    println!("{}", s);
}
```
This exercise covers basic String manipulation using push_str and push. It also introduces the concept of ownership and how to use clone() to allow continued use of the original String after the move_ownership function takes ownership of a copy.

## Exercise 2: String vs. &str

```rust
fn main() {
    // String is heap allocated, growable, and not null terminated.
    // &str is a slice that always points to a valid UTF-8 sequence.

    let mut s = String::from("hello, world");
    let slice = &s[0..1];

    assert_eq!(slice, "h");

    let slice2 = "hello";

    assert_eq!(slice2, "hello");

    let mut slice3 = String::from("hello, world");
    slice3.push('!'); // Modifies the String, not the &str

    assert_eq!(slice3, "hello, world!"); // This assert will FAIL because slice3 is modified.
    println!("Success");
}
```
This exercise highlights the key differences between String and &str.  It demonstrates how to create string slices (&str) from String and emphasizes that &str are immutable views into a string, while String is growable.  The example also demonstrates how modifying a String does not affect existing &str slices.  The assert will fail as is, but the exercise is designed to show that &str does not change when the underlying String is modified.

## Exercise 3: String Creation and Slices

```rust
fn main() {
    // Question: how many heap allocations are happening here?
    // Your answer: 2 (one for each String creation)

    let s: String = String::from("hello, world!"); // 1st allocation

    let slice: &str = &s; // No allocation, just a reference

    let s2: String = slice.to_string(); // 2nd allocation

    assert_eq!(s2, "hello, world!");

    println!("Success!");
}
```
This exercise focuses on string creation and the relationship between String and &str. It asks about the number of heap allocations (two) and demonstrates how to create a String from a &str slice.

## Exercise 4: UTF-8 and String Indexing

```rust
fn main() {
    let s = String::from("hello, 世界");

    let slice = &s[..1];
    assert_eq!(slice, "h");

    let slice2 = &s[7..10]; // Indices are byte indices, not character indices
    assert_eq!(slice2, "世"); // Correctly extracts the Chinese character (3 bytes)

    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世');
        }
    }

    println!("Success");
}
```
This exercise emphasizes that string indexing in Rust is based on bytes, not characters. It demonstrates how to correctly access UTF-8 characters using the chars() method.

## Exercise 5: String Creation from UTF-8 Bytes

```rust
fn main() {
    let mut s = String::new();
    s.push_str("hello");

    let v = vec![104, 101, 108, 108, 111];
    let s1 = String::from_utf8(v).unwrap(); // Converts a Vec<u8> to a String

    assert_eq!(s, s1);

    println!("Success");
}
```
This exercise shows how to create a String from a vector of UTF-8 encoded bytes using String::from_utf8.

## Exercise 6: String Capacity
```rust
fn main() {
    let mut s = String::with_capacity(25); // Creates a String with initial capacity

    println!("{}", s.capacity()); // Prints 25

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity()); // Capacity might increase as needed
    }

    println!("Success");
}
```
This exercise introduces the concept of string capacity using String::with_capacity. It shows how the capacity can change as you add more data to the string.  The capacity may increase to accommodate the string, but the capacity will never be less than the length of the string.