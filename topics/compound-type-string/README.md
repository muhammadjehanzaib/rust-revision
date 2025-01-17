# Understanding Strings in Rust

This document explores the different string types in Rust: `String`, `str`, and `&str`, along with practical exercises demonstrating their usage.

## String Types in Rust

Rust has three main ways to represent strings:

*   **`String`:** A growable, mutable, owned string. It's stored on the heap.
*   **`str`:** An immutable sequence of UTF-8 bytes. It's the most primitive string type, but you rarely use it directly.
*   **`&str`:** A string slice, which is a reference to a `str`. It's immutable and doesn't own the underlying string data. String literals like `"hello, world"` are of type `&str`.

## Exercises

Here are the exercises from the provided Rust code, explained and corrected:

### Exercise 1

```rust
let _s: &str = "Hello world";
println!("Success");
```

This exercise demonstrates the basic usage of a string literal, which is of type &str. It's already correct.

### Exercise 2
```rust
let s: Box<str> = "hello, world".into();
greeting(&s);

#[allow(dead_code)]
fn greeting(s: &str) {
    println!("{}", s);
}
```
This shows how to create a Box<str>. Box<T> is a smart pointer that allocates memory on the heap. Using .into() converts the &str literal into a Box<str>. The greeting function takes a &str, so we use &s to create a slice reference to the boxed string.

### Excercise 3

```rust
let mut s = String::from(""); // Or let mut s: String = "".to_string();
s.push_str("Hello, world");
s.push('!');

assert_eq!(s, "Hello, world!");
println!("Success");
```
This demonstrates creating a mutable String using String::from("") or "".to_string(), appending to it with push_str (for strings) and push (for single characters).

### Excercise 4
```rust
let mut s = String::from("hello");
s.push(',');
s.push_str(" world");
s += "!";
println!("{}", s);
```
This shows more ways to modify a String: using push, push_str, and the += operator for string concatenation.

### Excercise 5
```rust
let s = String::from("I like dogs");
let s1 = s.replace("dogs", "cats");
assert_eq!(s1, "I like cats");
assert_eq!(s, "I like dogs"); // s is not modified by replace
println!("Success");
```
This demonstrates the replace method, which creates a new String with the replacement. The original String (s) remains unchanged.

### Exercise 6 (Correctness)

```rust
let s1 = String::from("hello,");
let s2 = String::from("world"); // Corrected: s2 should be a String
let s3 = s1 + &s2; // Corrected: s2 needs a & to be a &str
assert_eq!(s3, "hello,world");
println!("Success");
```

This exercise was the most problematic in the original code. The + operator for String concatenation takes a &str as its right-hand operand. Therefore, s2 needs to be converted to a &str using &s2. Also, s2 should be a String, not &String. There was also a missing s4 variable that has been corrected to s3.

### Exercise 7

```rust
let s = "Hello, world";
greeting(String::from(s)); // or s.to_string() or s.to_owned()
```

Explanation:

This shows how to convert a &str to a String using String::from(), to_string(), or to_owned().

### Exercise 8

```rust
let s = "hello, world".to_string();
let _s1: &str = s.as_str(); // or &s[..]
println!("Success");
```
Explaination:
This demonstrates how to get a &str view of a String using as_str() or &s[..].

### Exercise 9

```rust
let byte_escape = "I'm writing Ru\x73\x74!";
println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);
```

This shows byte escapes in strings (\xHH, where HH is a hexadecimal number).


### Exercsie 10

```rust
let raw_str = r"Escapes don't work here: \x3F \u{211D}";
println!("{}", raw_str);
```
Explanation:

This demonstrates raw strings (r"..."), where escape sequences are treated literally.


### Exercise 11

```rust
let s1 = String::from("hi,中国");
let h = &s1[0..1];
assert_eq!(h, "h");

let h1 = &s1[3..6]; // 中 takes 3 bytes in UTF-8
println!("{}", h1);
```

Explanation:

This shows string slicing with UTF-8 characters. "中" takes 3 bytes, so the slice [3..6] is needed to get it.

### Exercise 12

```rust
for c in "你好，世界".chars() {
    println!("{}", c);
}
```
Explanation:

This demonstrates iterating over characters in a string using the chars() method, which correctly handles UTF-8.