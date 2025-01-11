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
    let s = give_ownership();
    println!("{}", s);
}

#[allow(dead_code)]
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.clone().into_bytes();
    s
}
