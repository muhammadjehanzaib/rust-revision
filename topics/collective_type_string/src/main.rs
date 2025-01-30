fn main() {
    // Basic Operations
    // Exercise 1
    // let mut s: String = String::from("Hello");
    // s.push_str(", world");
    // s.push('!');

    // move_ownership(s.clone());
    // assert_eq!(s, "Hello, world!");
    // println!("Success");

    // Exercise
    // String and str
    // A String is stored as a vector of bytes (Vec<u8>), but guaranteed to always be a valid UTF-8 sequence.
    // String is heap allocated, growable and not null terminated.
    // &str is a slice (&[u8]) that always points to a valid UTF-8 sequence,
    // and can be used to view into a String, just like &[T] is a view into Vec<T>.

    // let mut s = String::from("hello, world");
    // let slice = &s[0..1];
    // assert_eq!(slice, "h");

    // let slice2 = "hello";
    // assert_eq!(slice2, "hello");

    // let mut slice3 = String::from("hello, world");
    // slice3.push('!');
    // assert_eq!(slice3, "hello, world!");
    // println!("Success");

    // Exercise 3

    // Question: how many heap allocations are happening here?
    // Your answer: 2
    // Create a String type based on `&str`
    // The type of string literals is `&str`
    // let s: String = String::from("hello, world!");

    // // Create a slice point to String `s`
    // let slice: &str = &s;

    // // Create a String type based on the recently created slice
    // let s: String = slice.to_string();

    // assert_eq!(s, "hello, world!");

    // println!("Success!");

    // Exercise 4
    // let s = String::from("hello, 世界");
    // let slice = &s[..1];
    // assert_eq!(slice, "h");

    // let slice2 = &s[7..10];
    // assert_eq!(slice2, "世");

    // for (i, c) in s.chars().enumerate() {
    //     if (i == 7) {
    //         assert_eq!(c, '世');
    //     }
    // }
    // println!("Success");

    // Exercsie 5

    // let mut s =String::new();
    // s.push_str("hello");

    // let v = vec![104, 101, 108, 108, 111];
    //     // Turn a byte's vector into a String
    // let s1 = String::from_utf8(v).unwrap();
    // assert_eq!(s, s1);
    // println!("Success");

    // Exercise 6
    let mut s = String::with_capacity(25);
    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity())
    }
    println!("Success");
}

fn move_ownership(s: String) {
    println!("{}", s);
}
