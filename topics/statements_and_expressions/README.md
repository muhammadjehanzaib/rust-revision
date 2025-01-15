# Rust Code Explanation
All excercises are covered from rust by practice.

This Rust program demonstrates various concepts, including statements and expressions, and exercises to reinforce those concepts.

## Code Breakdown

### Statements and Expressions

1. **Variables and Scopes**:  
   In the following block, the code demonstrates the use of expressions in blocks and variable assignment.

    ```rust
    // let x = 5u32;

    // let y = {
    //     let x_square = x * x;
    //     let x_cube = x_square * x;
    //     x_cube + x_square + x
    // };
    ```

    Here, `y` is a result of an expression inside a block. The block calculates the square and cube of `x`, and returns their sum. Rust blocks return the value of the last expression in them.

2. **Statements with Semicolons**:  
   The following block shows a statement that uses a semicolon, meaning no value is returned:

    ```rust
    let z = {
        2 * x; // the semicolon here makes this a statement
    };
    ```

    The expression `2 * x` is evaluated but does not return any value because of the trailing semicolon. This results in the value of `z` being `()` (unit type).

3. **Printing the Values**:  
   This prints the values of `x`, `y`, and `z`:

    ```rust
    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
    ```

### Exercise 1: Explicit Return

In this exercise, you are asked to explicitly return a value from a block, rather than relying on Rust's implicit return. The following block illustrates this:

```rust
let v:u32 = {
    let mut x:u32 = 1;
    x += 2; // we can't directly return this expression
    x // explicit return (no semicolon here)
};
