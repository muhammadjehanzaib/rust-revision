# Rust Array Exercises and Solutions

This document provides solutions and explanations for the Rust array exercises presented in the provided code.

## Array Exercises

### Exercise 1

```rust
let arr = [1, 2, 3, 4, 5];
assert_eq!(arr.len(), 5);
println!("Success");
```

Explanation:

This exercise demonstrates the creation of a simple array and using the .len() method to get its length.


### Exercise 2
```rust
let arr: [char; 3] = ['a', 'b', 'c'];
assert_eq!(std::mem::size_of_val(&arr), 12);
println!("Success");
```

Explanation:

This exercise shows how to explicitly define the type of an array ([char; 3]). std::mem::size_of_val(&arr) returns the size of the array in bytes. Since each char is 4 bytes in Rust, an array of 3 chars occupies 12 bytes.


### Exercsie 3
```rust
let list: [i32; 100] = [1; 100];

assert!(list[0] == 1);
assert!(list.len() == 100);
println!("Success");
```
Explanation:

This demonstrates initializing an array with the same value for all elements using the syntax [value; size].

### Exercise 4
```rust 
let _arr = [1, 2, 3];
println!("Success");
```
Explanation:

This exercise implicitly shows that all elements in an array must have the same type. The code compiles because all elements are i32.

### Exericse 5

```rust
let arr = ['a', 'b', 'c'];
let ele = arr[0];

assert!(ele == 'a');
println!("Success");
```
Explanation:

This exercise shows how to access elements of an array using indexing (arr[index]).

### Exercise 6

```rust
let names = [String::from("Sunfei"), "Sunface".to_string()];
let _name0 = names.get(0).unwrap();

let _name1 = &names[1];
println!("Success");
```
Explanation:

This exercise demonstrates two ways to access elements of an array:

names.get(0).unwrap(): This is a safe way to access elements. get() returns an Option. If the index is out of bounds, it returns None. unwrap() extracts the value from the Option or panics if it's None.
&names[1]: This is direct indexing. If the index is out of bounds, it will cause a panic at runtime. Using get is generally preferred for safety.
Key takeaway about Exercise 6: While the exercise compiles and prints "Success", it highlights an important point about array access in Rust. Direct indexing (names[1]) can lead to panics if the index is out of bounds. The get() method with unwrap() (or better yet, handling the Option with match or if let) provides a safer alternative.

This improved documentation provides clear explanations for each exercise and expands on the important point about safe array access in Exercise 6. It also uses Markdown formatting for better readability.