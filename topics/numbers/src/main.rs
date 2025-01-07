fn main() {
    // Numbers
    // let's begin with integer

    // 1st exercise

    // let x: i32 = 5;
    // let mut y: u32 = 6; //we define the u32 data type here, and in the next line we are assigning value to i32 value.
    // we can't do this if can only operate between the same typr of varibles
    // lets modify line to make it works

    // let mut _y:i32 = 6;
    // _y = x;
    // let _z = 10;
    // println!("Success.")

    //2nd exercise
    // let v: u16 = 38_u8; // this is not going to work we must have to convert it by u16 data type to make it works
    // let _v:u16 = 38_u8 as u16;
    // println!("success");

    //3rd exercise
    // let x = 5; // default data type is i32
    // assert_eq!("i32".to_string(), type_of(&x));
    // println!("Success");

    // exercise 4
    // default value of i8 is -128 to 127
    // and u8 is 0 - 255
    // assert_eq!(i8::MAX, 127);
    // assert_eq!(u8::MAX, 255);
    // println!("Success");

    //exercise 5
    // let v1 = 251_u32 + 8_u32;
    // let v2 = u32::checked_add(251, 8).unwrap();
    // println!("{} , {}", v1, v2);

    // exercise 6
    //       decimal + hex  + octal + binary
    // let v = 1_024 + 0xff + 0o77  + 0b1111_1111;
    // this indicates that we can perform operation between diff number systems
    // assert!(v == 1597);
    // println!("Success");

    // Floating-Points
    // exercise 7
    //Default data type of the float variable f64

    // let x = 1_000.000_1; // f64
    // let y: f32 = 0.12; // f32
    // let z = 0.01_f64; // f64

    // assert_eq!(type_of(&x), "f64".to_string());
    // println!("Success");

    // exercise 8
    // what would be the output of the code, true? not exactly, so we have to understand the pattern. defualt value might be look like 0.300000000001 which is not equal to 0.3. to overcome this issue we must have to specify data type

    // assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
    // println!("success");

    // Rang
    // exercise 9
    // 🌟🌟 Two goals: 1. Modify assert! to make it work 2. Make println! output: 97 - 122
    // let mut sum:i32 = 0;
    // for i in -3..2 { // -3 included and 2 excluded here, to include 2 we must have to add = signt like -3..=2
    //     sum += i;
    // }
    // assert!(sum == -5);

    // for c in 'a'..='z'{ // print lowercase abc from a to z
    //     // println!("{}", c);  // it will print abc a to z
    //     println!("{}", c as u32); // it will print ascii values of a to z
    // }

    // Computations
    // exercise 11
    // integer addition
    assert!(1u32 + 2u32 == 3u32);
    // integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);
    // multiply
    assert!(3 * 50 == 150);
    assert!(9.6 / 3.2 != 3.0); // disscussed in floating-point exercise why it's true

    assert!(24 % 5 == 4);

    assert!(true && true == true);
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
#[allow(dead_code)] // it let us declare function without calling it
                    // This function is used ot check the data type of the give variavle and returnt it to caller
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
