## Rust Playground: Traits, Generics, and Dynamic Dispatch

This document explores various concepts in Rust, including traits, dynamic dispatch, and generics through a series of exercises.

**Exercise 1: Traits and Polymorphism**

```rust
trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
struct Swan;

impl Bird for Duck {
    fn quack(&self) -> String {
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String {
        "swan swan".to_string()
    }
}

fn hatch_a_bird(species: u8) -> Box<dyn Bird> {
    match species {
        1 => Box::new(Swan {}),
        2 => Box::new(Duck {}),
        _ => panic!(), 
    }
}

fn main() {
    let bird = hatch_a_bird(2);
    assert_eq!(bird.quack(), "duck duck");
    let bird = hatch_a_bird(1);
    assert_eq!(bird.quack(), "swan swan");
    println!("Success!");
}
```
This exercise demonstrates the use of traits and dynamic dispatch.
The Bird trait defines a common interface (quack) for different bird types. 
The hatch_a_bird function returns a boxed trait object (Box<dyn Bird>), allowing for polymorphism.

## Exercise 2: Working with a Slice of Trait Objects

```rust
trait Bird {
    fn quack(&self);
}

struct Duck;
impl Duck {
    fn fly(&self) {
        println!("Look, the duck is flying")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) {
        println!("{}", "duck duck");
    }
}

impl Bird for Swan {
    fn quack(&self) {
        println!("{}", "swan swan");
    }
}

fn main() {
    let birds: [Box<dyn Bird>; 2] = [Box::new(Duck {}), Box::new(Swan {})];

    for bird in birds {
        bird.quack();
        // when duck and swan turn into Bird, they all forget how to fly, and only remember how to quack
        // so, the below code will cause an error
        // bird.fly();
    }
}
```
This exercise shows how to create a slice of trait objects. 
This allows you to work with a collection of different types that implement the same trait.

## Exercise 3: Static vs. Dynamic Dispatch
```rust

trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

fn main() {
    let x = 1.1f64;
    let y = 8u8;

    // draw x
    draw_with_box(Box::new(x));

    // draw y
    draw_with_ref(&y);
}

fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}

fn draw_with_ref(x: &dyn Draw) {
    x.draw();
}
```
This exercise illustrates the difference between static and dynamic dispatch. static_dispatch uses generic types and performs dispatch at compile time, while dynamic_dispatch uses trait objects and performs dispatch at runtime.

# Exercise 4: Implementing MyTrait for Different Types

```rust
trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

// implement below with generics
fn static_dispatch<T: Foo>(x: T) {
    x.method();
}

// implement below with trait objects
fn dynamic_dispatch(x: &dyn Foo) {
    x.method();
}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    static_dispatch(x);
    dynamic_dispatch(&y);

    println!("Success!")
}
```
This exercise demonstrates how to implement a trait for different types (u8 and String) and use it with a generic function.

## Object safe

```rust
trait MyTrait {
    fn f(&self) -> Self;
}

impl MyTrait for u32 {
    fn f(&self) -> u32 { 42 }
}

impl MyTrait for String {
    fn f(&self) -> String { self.clone() }
}

fn my_function(x: impl MyTrait) -> impl MyTrait  {
    x.f()
}

fn main() {
    my_function(13_u32);
    my_function(String::from("abc"));
}
```