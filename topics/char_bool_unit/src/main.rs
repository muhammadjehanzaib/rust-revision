#[allow(unused_imports)] // it allow us to import without using imports
use std::mem::size_of_val;
fn main() {
    //Char
    // excersice 1

    // let c1 = 'a'; // single quotes for char and double qoutes for word
    // assert_eq!(size_of_val(&c1), 4); // we know the size of char is 4 bytes

    // // let c2 = "中"; // double quotes are confusing it's string and it is not going to compile
    // let c2 = '中'; // 4 bytes and string 16 bytes
    // assert_eq!(size_of_val(&c2), 4);

    // println!("success");

    // excersice 2
    // let c1 = "中"; // its string and passing it to as string
    // let c1 = '中';
    // print_char(c1);

    //boolean
    //excersice 3
    // let _f:bool = false;
    // let t = true;
    // if t{
    //     println!("success!");
    // }


    // excersice 4
    // let f = true;
    // let t = true && true;
    // assert_eq!(f,t);
    // println!("success");

    // unit type
    // excersice 5
    
    // let _v: () = ();

    // let v: (i32, i32) = (2,3);
    // assert_eq!(_v , implicitly_ret_unit()); // () == ()
    // println!("success");

    // excersice 6
    // the size of unit type is 0

    let uint: () = ();
    // assert!(size_of_val(&uint) == 4); // it wouldn't work becuase the size of unit is 0
    assert!(size_of_val(&uint) == 0);
    println!("success");

}

#[allow(dead_code)] // it allow us to write unused code without warning
fn implicitly_ret_unit() { // when a function dosen't return anything it returns unittype ()
    println!("i will return a ()");
}



#[allow(dead_code)] // it allow us to write unused code without warning
fn print_char(c: char) {
    println!("{}", c);
}
