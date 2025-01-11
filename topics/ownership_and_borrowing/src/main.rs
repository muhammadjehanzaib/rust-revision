fn main() {
    // Ownership
    // exercise 1
    // Use as many approaches as you can to make it work
    // let x = String::from("Hello world");
    // let y = x.clone();
    // println!("{}, {}", x, y);

    // exercise 2
    let s1 = String::from("hello world!");
    let s2 = take_ownership(s1);
    println!("{}", s2);

    //exercise 3
}

#[allow(dead_code)]
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}
