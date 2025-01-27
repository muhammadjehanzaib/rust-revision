## Rust Playground Exercises

This document explores various exercises demonstrating concepts like traits, generics, operator overloading, and more in Rust.

### Exercise 1: Traits for Greetings (Hello Trait)

```rust
trait Hello {
    fn say_hi(&self) -> String;
    fn say_something(&self) -> String;
}

struct Student {}
struct Teacher {}

impl Hello for Student {
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    fn say_something(&self) -> String {
        String::from("I'm a good student")
    }
}

impl Hello for Teacher {
    fn say_hi(&self) -> String {
        String::from("Hi, I'm your new teacher")
    }

    fn say_something(&self) -> String {
        String::from("I'm not a bad teacher")
    }
}

fn main() {
    let s = Student {};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t = Teacher {};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");

    println!("Success!");

}
```
This exercise defines a Hello trait with two methods: say_hi and say_something. Student and Teacher structs implement this trait, providing their unique greetings.

# Exercise 2: Units of Measurement with Traits

```rust 
   let _one_second = Seconds(1);
    println!("One Second look like {:?}", _one_second);

    let _this_is_true = _one_second == _one_second;
    let _this_is_false = _one_second > _one_second;

    let foot = Inches(12);
    println!("one foot equal {:?}", foot);
    let meter = Centimeters(100.0);
    let cmp =
        if foot.to_centimeters() < meter {
            "smaler"
        }else {
            "larger"
        };
        println!("one foot is {} than one meter", cmp);
// ,...........................................................
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// Inches
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters{
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Seconds(i32);
```

This exercise demonstrates units of measurement. It defines Inches and Centimeters structs to represent lengths. The Inches struct has a method to_centimeters to convert inches to centimeters.

# Exercise 3: Generic Multiplication Function
```rust
fn multiply<T>(x:T, y:T) -> T
    where T:std::ops::Mul<Output = T>,
    {
        x * y
    }

fn main() {
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));

    println!("Success!");
}
```
#### Or, Both are the same
```rust
fn multiply<T: std::ops::Mul<Output = T>>(a:T, b:T) -> T {
    a * b
}
```

# Exercise 4: Operator Overloading (commented out)

```rust
struct Foo;
struct Bar;
#[derive(PartialEq, Debug)]
struct FooBar;
#[derive(PartialEq, Debug)]
struct BarFoo;

impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        FooBar
    }
}

impl ops::Sub<Bar> for Foo {
    type Output = BarFoo;

    fn sub(self, _rhs: Bar) -> BarFoo {
        BarFoo
    }
}

// main
fn main() {
    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);

    println!("Success!");
}
```

## Exercise 5: Trait Implementation for Structs

```rust
trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post {} is {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{} published a weibo {}", self.username, self.content)
    }
}

fn summary(a: &impl Summary) {
    let output = a.summarize();
    println!("{}", output);
}

fn main() {
    let post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };

    summary(&post);
    summary(&weibo);

    println!("{:?}", post);
    println!("{:?}", weibo);
}
```

## Exercise 6: Random Animal with Dynamic Dispatch

```rust
struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> String;
}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaaah".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "mooooo!".to_string()
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random = 0.2545;
    let animal = random_animal(random);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}
```


## Exercise 7: Sum Function

```rust
fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

fn main() {
    assert_eq!(sum(1, 3), 4);
    println!("Success!");
}
```


## Exercise 8: Generic Struct with Comparison

```rust
#[derive(PartialEq, PartialOrd, Debug)]
struct Unit(i32);

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The larger number is x: {:?}", self.x);
        } else {
            println!("The larger number is y: {:?}", self.y);
        }
    }
}

fn main() {
    let pair = Pair {
        x: Unit(1),
        y: Unit(3),
    };
    pair.cmp_display();
}
```