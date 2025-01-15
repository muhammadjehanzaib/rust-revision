# Rust Code Exercises
All excercises are covered from rust by practice.

## Exercise 1: Functions and Assertions

```rust
fn main() {
    // Exercise 1: Using Functions
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);
    println!("Success");
}

#[allow(dead_code)]
fn sum(x: i32, y: i32) -> i32 {
    x + y
}
```
**Objective**: Create a function sum that adds two numbers and return the sum.
**Explanation**: The function sum takes two i32 integers as arguments and returns their sum. In the main function, the sum of 1 and 2 is computed, and the result is compared to 3 using assert_eq!.

## Exercise 2: Simple Print Function

```rust
fn main() {
    // Exercise 2: Print a message
    print();
}

#[allow(dead_code)]
fn print() -> () {
    println!("success");
}
```
**Objective**: Define a simple function to print "success".
**Explanation**: The print function is called in the main function, which prints "success" to the console.

## Exercise 3: Diverging Function (Panic)

```rust
fn main() {
    // Exercise 3: Using Diverging Function
    // Don't let `println!` work
    never_retrurn();
    println!("failed to panic");
}

#[allow(dead_code)]
fn never_retrurn() -> ! {
    panic!("panic on purpose");
}
```
**Objective**: Create a function that panics, preventing further execution.
**Explanation**: The never_retrurn function has a return type !, which is the "never" type (indicating it never returns). The function explicitly panics, and execution halts before any further code is executed, including println!.

## Exercise 5: Match Expression with Diverging Function
```rust
fn main() {
    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expressions
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}
```
**Objective**: Use a diverging function in a match expression.
**Explanation**: The match expression uses panic! as a diverging function to handle the case where b is false. If the match for false is executed, the program will panic before printing the last line.