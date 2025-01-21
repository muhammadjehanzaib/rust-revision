fn main() {
    // Slice
    // Slices are similar to arrays, but their length is not known at compile time, so you can't use slice directly.
    // Exercise 1
    // let arr: [i32; 3] = [1,2,3];
    // let _s1: &[i32] = &arr[0..2];

    // let _s2: &str = "hello, world";
    // println!("Success");

    // Exercise 2
    // let arr: [char; 3] = ['中', '国', '人'];

    // let slice = &arr[..2];

    // assert!(std::mem::size_of_val(&slice) == 16); // Corrected assertion

    // println!("Success!");
    // assert!(std::mem::size_of_val(&slice) == 8);
    // This is an assertion. std::mem::size_of_val(&slice) returns the size of the slice reference itself,
    // not the size of the data it points to. A slice in Rust is represented internally as a pointer to the start of the data and a length.
    // On a 64-bit system (which is very common), a pointer is 8 bytes, and the length (a usize) is also 8 bytes.
    // Thus, the size of the slice reference is 8 bytes (pointer) + 8 bytes (length) = 16 bytes, not 8.

    // Exercise 3
    // let arr: [i32;5] = [1,2,3,4,5];
    // let slice: &[i32] = &arr[1..4];
    // assert_eq!(slice, &[2,3,4]);
    // println!("Success");

    // Exercise 4
    // String slices
    // let s = String::from("hello");
    // let slice1 = &s[0..2];
    // let slice2 = &s[..2];

    // println!("Success");

    // Exercise 5
    // let s = "你好，世界"; // unicode 3 bytes in memory to store
    // let slice = &s[0..3];

    // assert!(slice == "你");
    // println!("Success");
    
    // Exercise 6
    // 🌟🌟 &String can be implicitly converted into &str.
    let mut s = String::from("Hello, world");

    let letter: &str = first_letter(&s); 

    println!("{:?}", letter);
    s.clear(); // this clear will remove all content("hello, world") from s. 
}

fn first_letter(s: &str) -> &str {
    &s[..1]
}