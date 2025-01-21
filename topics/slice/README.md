# Rust Slices Exercises

This document explains several exercises demonstrating the use of slices in Rust. Slices provide a way to reference a contiguous sequence of elements in a collection without owning the underlying data.

## Exercise 1: Basic Slices

```rust
// let arr: [i32; 3] = [1,2,3];
// let _s1: &[i32] = &arr[0..2];

// let _s2: &str = "hello, world";
// println!("Success");
```
This exercise introduces basic array and string slices. &arr[0..2] creates a slice of the array arr containing the first two elements. "hello, world" is a string literal, which is already a &str (string slice).

Exercise 2: Size of Slice References
```rust
let arr: [char; 3] = ['中', '国', '人'];

let slice = &arr[..2];

assert!(std::mem::size_of_val(&slice) == 16); // Corrected assertion

println!("Success!");
assert!(std::mem::size_of_val(&slice) == 8); // Incorrect

// This is an assertion. std::mem::size_of_val(&slice) returns the size of the slice reference itself,
// not the size of the data it points to. A slice in Rust is represented internally as a pointer to the start of the data and a length.
// On a 64-bit system (which is very common), a pointer is 8 bytes, and the length (a usize) is also 8 bytes.
// Thus, the size of the slice reference is 8 bytes (pointer) + 8 bytes (length) = 16 bytes, not 8.
```
Markdown

# Rust Slices Exercises

This document explains several exercises demonstrating the use of slices in Rust. Slices provide a way to reference a contiguous sequence of elements in a collection without owning the underlying data.

## Exercise 1: Basic Slices

```rust
// let arr: [i32; 3] = [1,2,3];
// let _s1: &[i32] = &arr[0..2];

// let _s2: &str = "hello, world";
// println!("Success");
```
This exercise introduces basic array and string slices. &arr[0..2] creates a slice of the array arr containing the first two elements. "hello, world" is a string literal, which is already a &str (string slice).

### Exercise 2: Size of Slice References

```rust
let arr: [char; 3] = ['中', '国', '人'];

let slice = &arr[..2];

assert!(std::mem::size_of_val(&slice) == 16); // Corrected assertion

println!("Success!");
assert!(std::mem::size_of_val(&slice) == 8); // Incorrect

// This is an assertion. std::mem::size_of_val(&slice) returns the size of the slice reference itself,
// not the size of the data it points to. A slice in Rust is represented internally as a pointer to the start of the data and a length.
// On a 64-bit system (which is very common), a pointer is 8 bytes, and the length (a usize) is also 8 bytes.
// Thus, the size of the slice reference is 8 bytes (pointer) + 8 bytes (length) = 16 bytes, not 8.
```
This exercise emphasizes that std::mem::size_of_val(&slice) returns the size of the slice reference (pointer + length), which is 16 bytes on a 64-bit system, not the size of the data the slice points to. Each char in this example is 4 bytes, so two chars are 8 bytes, but that's not what size_of_val(&slice) measures.



### Exercise 3: Slicing Arrays of Integers

```rust 
let arr: [i32;5] = [1,2,3,4,5];
let slice: &[i32] = &arr[1..4];
assert_eq!(slice, &[2,3,4]);
println!("Success");
```
This exercise demonstrates slicing an array of integers. &arr[1..4] creates a slice containing elements at indices 1, 2, and 3 (exclusive of index 4).

### Exercise 4: String Slices from String
```rust
let s = String::from("hello");
let slice1 = &s[0..2];
let slice2 = &s[..2];

println!("Success");
```
This shows how to create string slices from a String. &s[0..2] and &s[..2] are equivalent and create a slice containing the first two characters of the string.

### Exercise 5: Unicode String Slices
```rust
let s = "你好，世界"; // unicode 3 bytes in memory to store
let slice = &s[0..3];

assert!(slice == "你");
println!("Success");
```
This exercise demonstrates slicing a Unicode string. Because "你" is a single Unicode character that takes 3 bytes in UTF-8 encoding, &s[0..3] correctly extracts it. It's important to slice string at character boundaries in Unicode to avoid panics.

### Exercise 6: String Slice Function and Borrowing

```rust
fn main() {
    // 🌟🌟 &String can be implicitly converted into &str.
    let mut s = String::from("Hello, world");

    let letter: &str = first_letter(&s); 

    println!("{:?}", letter);
    s.clear(); // this clear will remove all content("hello, world") from s. 
}

fn first_letter(s: &str) -> &str {
    &s[..1]
}
```
This exercise highlights a critical point about borrowing and lifetimes. The first_letter function takes a &str (string slice) as input and returns a &str. This works because &String can be coerced into &str.

The important point is that the returned slice is a borrow of the original string data. Therefore, you cannot modify or invalidate the original string (s) while the slice (letter) is still in use. If you uncomment the s.clear() line, the compiler will produce an error because you are trying to modify (s.clear()) data that is currently borrowed by letter. This demonstrates Rust's borrow checker in action, preventing data races and ensuring memory safety.



