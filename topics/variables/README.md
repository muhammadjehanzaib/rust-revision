# Rust Variables by Example

This document explores core Rust variable concepts through code examples and explanations.

## 1. Binding and Mutability

Variables in Rust must be initialized before use.

*   **Initialization and Assignment:**

    ```rust
    let x: i32 = 5; // Declares x as an i32 and assigns 5
    ```

*   **Unused Variables:** To suppress compiler warnings:

    *   Prefix with underscore:

        ```rust
        let _y: i32; // Unused variable
        ```

    *   Use `#[allow(unused_variables)]` at the file level.

*   **Mutability:** Use `mut` to allow modification:

    ```rust
    let mut x = 1;
    x += 2;
    assert_eq!(x, 3);
    ```

## 2. Scope and Shadowing

Scope defines a variable's visibility.

*   **Block Scope:** Curly braces `{}` create a scope:

    ```rust
    let x: i32 = 10;
    {
        let x: i32 = 5; // Shadows the outer x
        println!("Inner x: {}", x); // Output: Inner x: 5
    }
    println!("Outer x: {}", x); // Output: Outer x: 10
    ```

## 3. Destructuring

Unpacking data structures into variables.

*   **Tuple Destructuring with Mutability:**

    ```rust
    let (mut x, y) = (1, 2);
    x += 2;
    assert_eq!(x, 3);
    assert_eq!(y, 2);
    ```

*   **Destructuring Assignments:**

    ```rust
    let (x, y);
    (x, ..) = (3, 4); // Assigns 3 to x
    [.., y] = [1, 2]; // Assigns 2 to y
    assert_eq!([x, y], [3, 2]);
    ```