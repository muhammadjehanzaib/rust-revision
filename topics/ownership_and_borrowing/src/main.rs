fn main() {
    // Ownership
    // exercise 1
    // Use as many approaches as you can to make it work
    // let x = String::from("Hello world");
    // let y = x.clone();
    // println!("{}, {}", x, y);

    // exercise 2
    // let s1 = String::from("hello world!");
    // let s2 = take_ownership(s1);
    // println!("{}", s2);

    //exercise 3
    // let s = give_ownership();
    // println!("{}", s);

    // exercise 4
    // by using clone
    // let s = String::from("Hello World");
    // print_str(s.clone());
    // println!("{}", s);

    // exercise 5
    // let x: (i32, i32, (), &str) = (1, 2, (), "hello");
    // let y = x;
    // println!("{:?} , {:?}", x, y);

    // Mutability
    // mutability can be changed when ownership is transfered
    //exercise 6
    // let s = String::from("hello");
    // let mut s1 = s;

    // s1.push_str("world");
    // println!("Success!");

    // exercise 7
    let x: Box<i32> = Box::new(5);
    let mut y: Box<i32> = Box::new(1);

    *y = 4;

    assert_eq!(*x, 5);
    println!("Success");
}

#[allow(dead_code)]
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

#[allow(dead_code)]
fn print_str(s: String) {
    println!("{}", s);
}

#[allow(dead_code)]
// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.as_bytes();
    s
}
