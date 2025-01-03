# Rust HashMap Exercises

This document explores various aspects of using `HashMap` in Rust, including creation, insertion, retrieval, iteration, and capacity management.

**Exercise 1: Basic HashMap Operations**

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58);

    // Get returns an Option<&V>
    let score = scores.get("Sunface");
    assert_eq!(score, Some(&98));

    if scores.contains_key("Daniel") {
        // Indexing returns a value V
        let score = scores["Daniel"]; // Potential panic if key is not present
        assert_eq!(score, 95);
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), 3);

    for (name, score) in &scores {
        println!("The score of {} is {}", name, score);
    }
}
```
This exercise demonstrates basic operations on a HashMap:

- Creating a new HashMap
- Inserting key-value pairs
- Retrieving values using get() and indexing
- Checking for key existence using contains_key()
- Removing a key-value pair using remove()
- Iterating over key-value pairs

## Exercise 2: Creating HashMaps from Arrays

```rust
use std::collections::HashMap;

fn main() {
    let teams = [
        ("Chinese team", 100),
        ("Americans team", 10),
        ("France team", 50),
    ];

    let mut teams_map1 = HashMap::new();

    for team in &teams {
        teams_map1.insert(team.0, team.1);
    }

    // let teams_map2 = HashMap::from(teams); // Method 1
    let teams_map2: HashMap<&str, i32> = teams.iter().cloned().collect(); // Method 2

    assert_eq!(teams_map1, teams_map2);
    println!("Success");
}
```
This exercise explores different ways to create a HashMap from an array of tuples:

- Using a loop to insert each key-value pair.
- Using HashMap::from() directly.
- Using the iter() and collect() methods on the array.

## Exercise 3: or_insert() and or_insert_with()

```rust
use std::collections::HashMap;

fn random_stat_buff() -> u8 {
    42
}

fn main() {
    let mut player_state = HashMap::new();
    player_state.entry("health").or_insert(100);

    assert_eq!(player_state["health"], 100);

    player_state.entry("health").or_insert_with(random_stat_buff);
    assert_eq!(player_state["health"], 100);

    let health: &mut u8 = player_state.entry("health").or_insert(50);
    assert_eq!(health, &100);
    *health -= 50;
    assert_eq!(*health, 50);
    println!("Success");
}
```
This exercise demonstrates the use of or_insert() and or_insert_with methods to insert values into a HashMap only if the key is not already present.

## Exercise 4: Using Custom Types as Keys

```rust
use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    fn new(name: &str, country: &str) -> Self {
        Self {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn main() {
    let viking = HashMap::from([
        (Viking::new("Einar", "Norway"), 25),
        (Viking::new("Olaf", "Denmark"), 24),
        (Viking::new("Harald", "Iceland"), 12),
    ]);

    for (vikin, health) in &viking {
        println!("{:?} has {} hp", vikin, health);
    }
}
```
This exercise shows how to use custom structs as keys in a HashMap. To use a custom type as a key, it must implement Eq, Hash, and PartialEq.

## Exercise 5: Capacity and Shrinking

```rust
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);

    assert!(map.capacity() >= 100);

    map.shrink_to(50);
    assert!(map.capacity() >= 50);

    map.shrink_to_fit();
    assert!(map.capacity() >= 2); // Capacity may not be exactly 2, but should be at least 2

    println!("Success");
}
```
This exercise demonstrates how to control the capacity of a HashMap using with_capacity() and shrink_to(). shrink_to_fit() attempts to reduce the capacity to the minimum required to hold the current number of elements.

## Exercise 6: Ownership and Borrowing

```rust
use std::collections::HashMap;

fn main() {
    let v1 = 10;
    let mut m1 = HashMap::new();
    m1.insert(v1, v1); // `v1` is copied into the HashMap
    println!("v1 is still usable after inserting to hashmap : {}", v1);

    let v2 = "hello".to_string();
    let mut m2 = HashMap::new();
    // Ownership moved here
    m2.insert(&v2, v1); 

    assert_eq!(v2, "hello"); // This is still valid because we inserted a reference (&v2)

    println!("Success");
}
```
This exercise highlights the importance of ownership when using strings as keys in a HashMap. Inserting a String directly moves ownership to the HashMap, making the original String no longer usable. However, inserting a reference (&String) allows the original String to remain valid.