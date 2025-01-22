# Rust Enum Exercises

## Overview
This file contains a set of exercises to demonstrate various features and use cases of enums in Rust. Each exercise is designed to highlight specific enum capabilities, including pattern matching, explicit discrimination, and handling variants with data.

---

## Exercises

### Exercise 1: Explicit Discrimination
Enums can have explicitly defined discriminators.

#### Code Example:
```rust
// Enum without explicit discrimination
enum Number {
    Zero,
    One,
    Two,
}

// Enum with implicit discrimination (starts from 0)
enum Number1 {
    Zero = 0,
    One,
    Two,
}

// Enum with explicit discrimination
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}
fn main() {
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);
}
```

---

### Exercise 2: Enum Variants with Data
Enums can hold data in their variants.

#### Code Example:
```rust
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn main() {

    let msg1 = Message::Move { x: 1, y: 2 };
let msg2 = Message::Write(String::from("Hello, World"));

println!("{:?}", msg1);
println!("{:?}", msg2);
}
```

---

### Exercise 3: Pattern Matching with Enums
You can extract data from enum variants using pattern matching.

#### Code Example:
```rust
let msg = Message::Move { x: 1, y: 1 };

if let Message::Move { x: a, y: b } = msg {
    assert_eq!(a, b);
    println!("Success");
} else {
    panic!("Never let this run");
}

// OR using match
match msg {
    Message::Move { x: a, y: b } => {
        assert_eq!(a, b);
        println!("Success");
    },
    _ => panic!("Never let this run"),
}
```

---

### Exercise 4: Enum in Arrays
Enums can be stored in arrays and processed with loops.

#### Code Example:
```rust
let msgs: [Message; 3] = [
    Message::Quit,
    Message::Move { x: 1, y: 2 },
    Message::ChangeColor(255, 255, 0),
];

for msg in msgs {
    show_message(msg);
}

fn show_message(msg: Message) {
    println!("{:?}", msg);
}
```

---

### Exercise 5: Using `Option` with Enums
Enums like `Option` are commonly used to handle values that might or might not exist.

#### Code Example:
```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(v) => Some(v + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

if let Some(n) = six {
    println!("{}", n);
    println!("Success");
} else {
    panic!("Never let me run this");
}
```

---

## Key Concepts Covered
- **Explicit Discrimination:** Define specific values for enum variants.
- **Data in Variants:** Store and manage additional data with enums.
- **Pattern Matching:** Extract data or execute code based on the variant.
- **`Option` Handling:** Use enums like `Option` to manage nullable values.

---

## Notes
- Enums provide a versatile way to model different states and behaviors in Rust.
- Pattern matching is a powerful tool for working with enums and extracting their data.
- Using enums like `Option` and `Result` helps enforce safer code practices by handling nullable or fallible operations explicitly.

---

## Resources
- [Rust Documentation: Enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
- [The Option Enum](https://doc.rust-lang.org/std/option/)
- [Pattern Matching](https://doc.rust-lang.org/book/ch06-02-match.html)
