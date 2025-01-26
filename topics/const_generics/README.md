# Rust Code Example: Arrays and Const Generics

## Code

```rust
fn main() {
    // Exercise 1: Array of custom `Array` structs
    let array = [
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [3, 4, 5],
        },
        Array {
            data: [6, 7, 8],
        },
    ];

    // Print each array in the `array` slice
    for arr in &array {
        print_array(arr.data);
    }

    // Exercise 2: Using `print_array` with different types
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", ",world"];
    print_array(arr);
}

// Generic function to print arrays of any type and size
fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

// Custom `Array` struct with const generics
struct Array<T, const N: usize> {
    data: [T; N],
}
```
# Explanation
## Exercise 1:

- We define an array of custom Array structs, where each Array contains an array of integers.

- The Array struct uses const generics (const N: usize) to allow arrays of any size.

- We iterate over the array slice and print each inner array using the print_array function.

## Exercise 2:

- We demonstrate the flexibility of the print_array function by calling it with:

    - An array of integers: [1, 2, 3].

    - An array of string slices: ["hello", ",world"].

### print_array Function:

- This is a generic function that takes an array of any type T and any size N.

- It uses the Debug trait (T: std::fmt::Debug) to print the array.

### Array Struct:

- This is a custom struct that uses const generics to store an array of type T with a fixed size N.