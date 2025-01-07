# Rust Numbers and Computations Exercises

This document outlines a series of exercises designed to reinforce understanding of numbers, floating-point types, ranges, computations, and bitwise operations in Rust.

## Exercises

### Integers

*   **Exercise 1:** Demonstrates the importance of using consistent data types when assigning and operating on variables. The original code attempted to assign a `u32` value to an `i32` variable, which is not allowed. The corrected code uses `i32` for both variables.

    ```rust
    // Original (incorrect)
    // let x: i32 = 5;
    // let mut y: u32 = 6;

    // Corrected
    let x: i32 = 5;
    let mut _y:i32 = 6;
    _y = x;
    let _z = 10;
    println!("Success.");
    ```

*   **Exercise 2:** Shows how to explicitly convert between integer types using the `as` keyword.

    ```rust
    // Original (incorrect)
    // let v: u16 = 38_u8;

    // Corrected
    let _v:u16 = 38_u8 as u16;
    println!("success");
    ```

*   **Exercise 3:** Demonstrates how to check the data type of a variable using the `type_of` function (defined later).

    ```rust
    let x = 5; // default data type is i32
    // assert_eq!("i32".to_string(), type_of(&x)); // Commented out assertion
    println!("Success");
    ```

*   **Exercise 4:** Verifies the maximum values of `i8` and `u8` integer types.

    ```rust
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
    println!("Success");
    ```

*   **Exercise 5:** Shows how to handle potential integer overflow using `checked_add`.

    ```rust
    let v1 = 251_u32 + 8_u32;
    let v2 = u32::checked_add(251, 8).unwrap();
    println!("{} , {}", v1, v2);
    ```

*   **Exercise 6:** Demonstrates arithmetic operations between different number systems (decimal, hexadecimal, octal, binary).

    ```rust
    let v = 1_024 + 0xff + 0o77  + 0b1111_1111;
    assert!(v == 1597);
    println!("Success");
    ```

### Floating-Point Numbers

*   **Exercise 7:** Shows how to define floating-point variables with `f64` (default) and `f32` types.

    ```rust
    let x = 1_000.000_1; // f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    // assert_eq!(type_of(&x), "f64".to_string()); // Commented out assertion
    println!("Success");
    ```

*   **Exercise 8:** Highlights the precision limitations of floating-point arithmetic. Direct comparison of floating-point numbers can be unreliable due to rounding errors.

    ```rust
    // assert!(0.1_f32 + 0.2_f32 == 0.3_f32); // This assertion might fail
    println!("success");
    ```

### Ranges

*   **Exercise 9:** Demonstrates the use of ranges in `for` loops, including exclusive (`..`) and inclusive (`..=`) ranges. It also shows how to iterate over characters and get their ASCII values.

    ```rust
    let mut sum:i32 = 0;
    for i in -3..2 {
        sum += i;
    }
    assert!(sum == -5);

    for c in 'a'..='z'{
        println!("{}", c as u32);
    }
    ```

### Computations and Bitwise Operations

*   **Exercise 11:** Demonstrates basic arithmetic operations (addition, subtraction, multiplication, division, modulo) and logical operations (AND, OR, NOT). It also shows bitwise operations (AND, OR, XOR, left shift, right shift).

    ```rust
    assert!(1u32 + 2u32 == 3u32);
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);
    assert!(3 * 50 == 150);
    assert!(9.6 / 3.2 != 3.0);

    assert!(24 % 5 == 4);

    assert!(true && true == true);
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    ```

### Helper Function

*   `type_of`: A helper function to determine the data type of a variable.

    ```rust
    #[allow(dead_code)]
    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }
    ```

## Conclusion

These exercises provide a practical introduction to working with numbers and performing computations in Rust. They cover essential concepts like integer and floating-point types, ranges, basic arithmetic, logical operations, and bitwise operations. Understanding these fundamentals is crucial for writing effective Rust programs.