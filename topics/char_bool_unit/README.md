# Rust Data Types: Characters, Booleans, and Unit Type Exercises
All excercises are covered from rust by practice.

This document covers exercises related to characters (`char`), booleans (`bool`), and the unit type `()` in Rust.

## Characters (`char`)

*   **Exercise 1:** Demonstrates the size of a `char` in Rust.

    ```rust
    use std::mem::size_of_val;

    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4); // char is 4 bytes

    let c2 = '中'; // Also 4 bytes (Unicode character)
    assert_eq!(size_of_val(&c2), 4);

    println!("success");
    ```

    **Explanation:** In Rust, a `char` represents a Unicode scalar value, which is a 32-bit (4-byte) value. This means it can represent a wide range of characters, including characters from various languages and symbols.

*   **Exercise 2:** Shows how to print a character.

    ```rust
    let c1 = '中';
    print_char(c1);

    #[allow(dead_code)]
    fn print_char(c: char) {
        println!("{}", c);
    }
    ```

## Booleans (`bool`)

*   **Exercise 3:** A simple `if` statement demonstrating the use of a boolean variable.

    ```rust
    let t = true;
    if t {
        println!("success!");
    }
    ```

*   **Exercise 4:** Demonstrates boolean logic using the `&&` (AND) operator.

    ```rust
    let f = true;
    let t = true && true;
    assert_eq!(f, t);
    println!("success");
    ```

## Unit Type `()`

*   **Exercise 5:** Shows the use of the unit type `()` and how it's implicitly returned by functions that don't have an explicit return value.

    ```rust
    let _v: () = (); // Explicitly assigning the unit type

    assert_eq!(_v, implicitly_ret_unit()); // () == ()
    println!("success");

    #[allow(dead_code)]
    fn implicitly_ret_unit() {
        println!("i will return a ()");
    }
    ```

    **Explanation:** The unit type `()` is a type that has only one possible value, also written as `()`. It represents the absence of a value. Functions that don't explicitly return a value implicitly return the unit type.

*   **Exercise 6:** Demonstrates that the size of the unit type is 0 bytes.

    ```rust
    use std::mem::size_of_val;

    let uint: () = ();
    assert!(size_of_val(&uint) == 0);
    println!("success");
    ```

    **Explanation:** The unit type `()` takes up no memory space. It's a zero-sized type.

## Helper Functions

*   **`implicitly_ret_unit()`:** A function that implicitly returns the unit type `()`.
*   **`print_char()`:** A function to print a character.

    ```rust
    #[allow(dead_code)]
    fn implicitly_ret_unit() {
        println!("i will return a ()");
    }

    #[allow(dead_code)]
    fn print_char(c: char) {
        println!("{}", c);
    }
    ```

## Key Learning Points

*   `char` in Rust represents a Unicode scalar value and is 4 bytes in size.
*   `bool` represents boolean values (true or false).
*   The unit type `()` represents the absence of a value and has a size of 0 bytes.
*   Functions without an explicit return type implicitly return the unit type.

This document provides exercises to understand fundamental data types in Rust. Experimenting and modifying the code is highly encouraged for better understanding.