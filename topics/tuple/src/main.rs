fn main() {
    // Exercise 1
    // Elements in a tupl can have different types. Tuple's type signature is (T1, T2, ...), where T1, T2 are the types of tuple's members.
    // let _t0: (u8, i16) = (2, -1);
    // // Tuples can have tuple's members
    // let _t1: (u8, (i16, i32)) = (3,(-3, 44));
    // let _t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
    // println!("Success!");

    // Exercise 2
    // Members can be extracted from the tuple using indexing
    // let t = ("i", "am", "sunface");
    // assert_eq!(t.2, "sunface");
    // println!("Success");

    // Exercise 3
    // Long tuples cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    // println!("too long tuple {:#?}", too_long_tuple);

    // Exercise 4
    // Destucturing Tuple with patterns
    // let tup = (1, 6.4, "hello");
    // let (x, z, y) = tup;

    // assert_eq!(x, 1);
    // assert_eq!(y, "hello");
    // assert_eq!(z, 6.4);

    // println!("Success!");

    // Exercise 5
    // let (x, y, z);
    // (x, y, z) = (3, 1, 2);

    // assert_eq!(x, 3);
    // assert_eq!(y, 1);
    // assert_eq!(z, 2);
    // println!("Success");

    // 🌟🌟 Tuples can be used as function arguments and return values
    let (x, y) = sum_multiply((2, 3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);
    println!("Success");
}

fn sum_multiply(num: (i32, i32)) -> (i32, i32) {
    (num.0 + num.1, num.0 * num.1)
}
