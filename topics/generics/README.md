# Rust Generics Exercises

This document explores generics in Rust, demonstrating their use in functions, structs, and methods. Generics allow you to write code that works with multiple types without code duplication.

## Exercise 1: Generic Functions

```rust
struct A;
struct S(A);
struct SGen<T>(T);

fn reg_fn(_s: S) {}
fn gen_spec_t(_s: SGen<A>) {}
fn gen_spec_i32(_s: SGen<i32>) {}
fn generics<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(8));
    // Explicitly
    generics::<char>(SGen('A'));
    // Implicitly
    generics(SGen('B'));
    println!("Success");
}
```
This exercise demonstrates various ways to use generic functions:

- reg_fn: A regular function that takes a specific type S.
- gen_spec_t: A generic function specialized for type A.
- gen_spec_i32: A generic function specialized for type i32.
- generics<T>: A fully generic function that can work with any type T. It shows both explicit (::<char>) and implicit type specification.

# Exercise 2: Generic sum Function with Trait Bounds
```rust
use std::ops::Add;

fn sum<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

fn main() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));
    println!("Success");
}
```
This exercise shows how to use trait bounds (T: Add<Output = T>) to restrict the types that can be used with a generic function. In this case, T must implement the Add trait, and the output of the addition must also be of type T.

# Exercise 3 & 4: Generic Structs
```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let interger = Point { x: 5, y: 9 };
    let float = Point { x: 1.0, y: 4.0 };
    let p = Point {
        x: 5,
        y: "hello".to_string(),
    };
    println!("Success");
}
```
These exercises demonstrate how to define structs with generic type parameters. Point<T, U> can hold values of different types for x and y.

# Exercise 5: Generic Structs and Methods
```rust
struct Val<T> {
    val: T,
}

impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = Val {
        val: String::from("Hello, it's me"),
    };
    println!("{} , {}", x.value(), y.value());
}
```
This exercise shows how to implement methods for generic structs. The value() method returns a reference to the value stored in the Val struct.

# Exercise 6: Generic Methods with Different Type Parameters

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point {
        x: "Hello",
        y: '中',
    };

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');
    println!("Success");
}
```

This exercise demonstrates a method (mixup) that takes another generic struct as an argument and returns a new struct with a mix of the original types.

# Exercise 7: Implementation Blocks with Specific Types

```rust 
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f64> {
    fn distance_form_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 5.0, y: 10.0 };
    println!("{}", p.distance_form_origin());
}
```
This exercise shows how to implement methods specifically for a certain type of generic struct (Point<f64>). The distance_form_origin() method is only available for Point structs where T is f64.