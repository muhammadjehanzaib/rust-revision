# Rust Match and if let Exercises

## Match Statements

### Exercise 1
```rust
let dire = Direction::South;
match dire {
    Direction::East => println!("East"),
    Direction::South | Direction::North => {
        println!("South or East");
    }
    _ => println!("West"),
}
```

### Exercise 2
Match as an expression:
```rust
let boolean = true;
let binary = match boolean {
    true => 1,
    _ => 0,
};
assert_eq!(binary, 1);
println!("Success");
```

### Exercise 3
Using match to retrieve data from enum variants:
```rust
let msgs = [
    Message::Quit,
    Message::Move { x: 1, y: 3 },
    Message::ChangeColor(255, 255, 0),
];
for msg in msgs {
    show_messages(msg);
}
println!("Success");

fn show_messages(msg: Message) {
    match msg {
        Message::Move { x: a, y: b } => {
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        }
        Message::ChangeColor(_, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        }
        _ => println!("No data in these variants"),
    }
}
```

### Exercise 4
Matching ranges:
```rust
let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];
for ab in alphabets {
    assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'));
}
println!("Success");
```

### Exercise 5
Counting matches:
```rust
let mut count = 0;
let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
for e in v {
    if matches!(e, MyEnum::Foo) {
        count += 1;
    }
}
assert_eq!(count, 2);
println!("Success");
```

## Using if let

### Exercise 6
```rust
let o = Some(7);
if let Some(x) = o {
    println!("this is really long string and {:?}", x);
}
```

### Exercise 7
```rust
let a = Foo::Bar(1);
if let Foo::Bar(x) = a {
    println!("foobar holds the value {}", x);
    println!("Success");
}
```

### Exercise 8
```rust
let a = Foo::Quz(10);
match a {
    Foo::Bar => println!("match foo::bar"),
    Foo::Baz => println!("match foo::baz"),
    _ => println!("match others"),
}
```

### Exercise 9
```rust
let age = Some(30);
if let Some(age) = age {
    assert!(age == 30);
}
match age {
    Some(age) => println!("age is a new variable, it's value is {}", age),
    _ => (),
}
```

## Enums Used
```rust
enum Foo {
    Bar,
    Baz,
    Quz(u32),
}

enum MyEnum {
    Foo,
    Bar,
}

enum Direction {
    East,
    West,
    North,
    South,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```