fn main() {
    // Reference
    // let x  = 5;
    // // Fill the blank
    // let p = &x;
    // println!("{:p}", p);

    // Exercise 2
    // let x = 5;
    // let y = &x; // reference to y

    // assert_eq!(5, *y); // d-reference to y
    // println!("Success");

    // Exercise 3
    // let mut s = String::from("hello, ");

    // borrow_object(&s);
    // println!("Success");

    // Exercise 4
    // let mut s = String::from("hello, ");
    // push_str(&mut s);
    // println!("Success");

    // Exercise 5
    // let mut s = String::from("hello, ");
    // let p = &mut s;
    // p.push_str("world");
    // println!("Success");

    // Exercise 6
    // Ref
    // ref can be used to take references to a value, similar to &.
    // Exercise 6
    // let x = 'V';
    // let r1 = &x;
    // let ref r2 = x;

    // assert_eq!(*r1, *r2);
    // // check the equality of the two address strings
    // assert_eq!(get_addr(r1), get_addr(r2));
    // println!("Success");

    // Borrowing rules
    // exercise 7
    // let mut s = String::from("hello");
    // let r1 = &mut s;

    // println!("{}", r1);
    // println!("Success");

    // exercise 8
    // Mutability
    // let mut s = String::from("hello, ");

    // borrow_object(&mut s);
    // println!("Success");

    // excercise 9
    // let mut s = String::from("hello ,");
    // borrow_object(&s);
    // s.push_str("world");
    // println!("Success");

    // excercise 10
    // let mut s = String::from("hello, ");

    // let r1 = &mut s;
    // r1.push_str("world");
    // // let r2 = &mut s;
    // // r2.push_str("!");

    // println!("{}", r1);

    // excersie 11
    // let mut s = String::from("hello, ");
    // let r1 = &mut s;
    // let r2 = &mut s;

    // r1.push_str("string");
    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time
}
#[allow(dead_code)]
fn borrow_object(_s: &String) {}

// #[allow(dead_code)]
// fn borrow_object(_s: &mut String) {}

#[allow(dead_code)]
fn push_str(s: &mut String) {
    s.push_str("world");
}
#[allow(dead_code)]
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}
