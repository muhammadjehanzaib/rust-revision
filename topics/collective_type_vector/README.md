# Rust Vector Exercises

This document provides a series of exercises to practice working with vectors in Rust, covering various operations such as creation, manipulation, slicing, capacity, and storing trait objects.

## Exercise 1: Vector Creation

```rust
fn main() {
    let arr: [u8; 3] = [1, 2, 3];

    let v = Vec::from(arr);
    is_vec(&v);

    let v = vec![1, 2, 3];
    is_vec(&v);

    let v = vec!(1, 2, 3);
    is_vec(&v);

    let mut v1 = Vec::new();
    v1.extend(arr);

    assert_eq!(v, v1);
    println!("Success");
}

fn is_vec(x: &Vec<u8>) {}
```

This exercise demonstrates different ways to create vectors: from an array using Vec::from(), using the vec![] macro, and using the vec!() macro.
It also shows how to extend an empty vector with elements from an array.

## Exercise 2: Vector Manipulation
```rust
fn main() {
    let mut v1 = Vec::from([1, 2, 3]);
    v1.pop(); // Removes the last element
    v1.push(3); // Adds an element to the end

    let mut v2 = Vec::new();
    v2.extend(&v1); // Copies all elements from v1 to v2

    assert_eq!(v1, v2);
    println!("Success");
}
```
This exercise covers basic vector manipulation using pop() to remove the last element, 
push() to add an element, and extend() to copy elements from one vector to another.

## Exercise 3: Conversions to Vectors
```rust

fn main() {
    // arr -> Vec
    let arr = [1, 2, 3];
    let v1 = Vec::from(arr);
    let v2: Vec<i32> = v1.clone().into(); // Cloning is necessary before ownership transfer

    assert_eq!(v1, v2);

    // String -> Vec
    let s = "hello".to_string();
    let v1: Vec<u8> = s.into(); // Ownership transfer

    let s = "hello".to_string();
    let v2 = s.into_bytes(); // More idiomatic way to get bytes from String

    assert_eq!(v1, v2);

    let s = "hello"; // &str
    let v3 = Vec::from(s); // Copies the &str data into a new Vec<u8>

    assert_eq!(v2, v3);

    let v4: Vec<i32> = [0; 10].into_iter().collect(); // From array iterator
    assert_eq!(v4, vec![0; 10]);

    println!("Success");
}
```
This exercise shows different ways to convert other data types to vectors: from arrays, from String to Vec<u8>, and using iterators. 
It's important to note the difference between moving ownership and cloning when converting.


## Exercise 4: Accessing and Modifying Elements

```rust

fn main() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        println!("{:?}", v.get(i)); // Option<i32> - Safe access
    }

    for i in 0..5 {
        match v.get(i) {
            Some(e) => v[i] = e + 1, // Safe mutable access if element exists
            None => v.push(i + 2),    // Add element if it doesn't exist
        }
    }

    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    for i in 0..5 {
        println!("{:?}", v.get(i));
    }

    println!("Success!");
}
```
This exercise demonstrates how to safely access vector elements using get()
 which returns an Option, and how to modify elements or add new ones based on whether they exist.  
 It also shows how to create a vector larger than the initial array.

# Exercise 5: Slices
```rust

fn main() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..]; // Entire vector
    let slice2 = &v[0..v.len()]; // Entire vector

    assert_eq!(slice1, slice2);

    let vec_ref: &mut Vec<i32> = &mut v; // Mutable reference to the vector
    (*vec_ref).push(4); // Modifies the original vector

    let slice3 = &v[0..4]; // Slice including the new element

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Success!");
}
```
This exercise focuses on vector slices (&[T]). It shows how to create slices that refer to the entire vector or a portion of it.
 It also highlights that slices are read-only views and that a &mut Vec can be used to modify the original vector.

## Exercise 6:Capacity
```rust
fn main() {
    let mut vec = Vec::with_capacity(10); // Initialize with capacity

    assert_eq!(vec.len(), 0); // Length is 0
    assert_eq!(vec.capacity(), 10); // Capacity is 10

    for i in 0..10 {
        vec.push(i); // No reallocations needed yet
    }

    assert_eq!(vec.len(), 10);
    assert_eq!(vec.capacity(), 10);

    vec.push(11); // Might cause reallocation
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11); // Capacity has grown

    let mut vec = Vec::with_capacity(100); // Initialize with capacity 100
    for i in 0..100 {
        vec.push(i); // No reallocations needed
    }

    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 100);

    println!("Success!");
}
```
This exercise explains the difference between vector length (number of elements) and capacity (allocated space).
It demonstrates how with_capacity() can be used to pre-allocate space and avoid reallocations as elements are added.


Exercise 7 & 8: Storing Trait Objects in Vectors
```rust

trait IpAddr {
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0)
    }
}

struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0)
    }
}

fn main() {
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}
```
These exercises demonstrate how to store trait objects (boxed trait objects) in a vector. 
This allows you to store different types that implement the same trait in a single vector and call their methods dynamically.








