# Rust Structs Exercises

This document explores various exercises demonstrating the use of structs in Rust. Structs are user-defined types that bundle related data fields under a single name.

## Exercise 1: Basic Struct Definition and Field Initialization

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    hobby: String,
}

let age: u8 = 30;
let p: Person = Person {
    name: String::from("Alice"),
    // age: age, correct but we can also use this
    age, // called "field init short hand" it is same as
    // It's similar to modern JavaScript object shorthand property names.
    // The compiler expands age to age: age automatically.
    hobby: String::from("Blockchain Dev"),
};
println!("{:?}", p);
println!("Success");
```

This exercise introduces how to define a struct named Person with three fields: name (a String), age (an unsigned 8-bit integer), and hobby (another String). It also demonstrates initializing a struct using field initialization shorthand and printing the struct content using {:?} formatting (which requires the Debug derive).

# Exercise 2: Unit Struct (Empty Struct)
```rust
let u = Unit;
do_something(u);
println!("Success");

struct Unit;
trait SomeTrait {
    // .... Some behaviours defined here ...
}
impl SomeTrait for Unit {}
fn do_something(_u: Unit) {}
```
This exercise showcases the Unit struct, which is a struct with no fields. It's often used as a placeholder type when a function requires a struct but doesn't need to store any data. The provided code also includes the commented-out definition of Unit and a trait SomeTrait that Unit could implement (illustrating how structs can be associated with traits).

# Exercise 3: Tuple Struct
```rust
let v: Point = Point(0, 127, 255);
check_color(v);
println!("Success");

struct Point(i32, i32, i32);
struct Color(i32, i32, i32);
fn check_color(p: Point) {
    // INCORRECT - Can't destructure struct like a tuple
    // let Point(x, y, z) = p; // Error
    // let (x, y, z) = (p.0, p.1, p.2);
    let Point(ref x, ref _y, ref z) = p;
    assert_eq!(*x, 0); // Dereferencing here
    assert_eq!(p.1, 127);
    assert_eq!(*z, 255);
}
```
This exercise demonstrates tuple structs. They look similar to tuples but have a name associated with them, providing more meaning. Here, Point is a tuple struct with three fields. The commented-out check_color function shows that you cannot destructure a struct like a tuple (e.g., let Point(x, y, z) = p). To access fields, you use dot notation (e.g., p.1).


# Exercise 4: Mutability in Structs
```rust
let age = 30;
let mut p = Person {
    name: String::from("Sunface"),
    age,
};

p.age = 20;
p.name = String::from("flying Curve");
println!("Success");

struct Person {
    name: String,
    age: u8,
}
```
This exercise highlights mutability in structs. By default, structs are immutable. You can make a struct mutable when creating an instance using let mut p. This allows modifying the fields of p after creation (e.g., p.age = 20).

# Exercise 5: Struct Function
```rust
let _p = build_person(String::from("hello"), 9);
println!("Success");

struct Person {
    name: String,
    age: u8,
}

fn build_person(name: String, age: u8) -> Person {
    Person {
        name,
        age,
    }
}
```
