// following Rust by example https://practice.course.rs/variables.html

fn main() {
    // Binding and mutability
    // 1 🌟 A variable can be used only if it has been initialized.

    // let x: i32 = 5; // initialized and assigned value 5 to variable x
    // let _y: i32; //  uninitialized and also unused, to remove warning by the complier we can do 2 things here, prefix y with _, like _y or use #[allow(unused_variables)] on the top of the file

    // assert_eq!(x, 5);
    // println!("Success....!");
    // ..........................................................................................................................................
    // 2 🌟 Use mut to mark a variable as mutable.

    // let mut x = 1; // if we want to modify the variable x, we must have to use key word mut(mutable) to let compiler know we will modify this vairable value in future.
    // x +=2; // here we are increamenting value by 2
    // assert_eq!(x, 3);
    // println!("Success!");
    //............................................................................................................................................
    // Scope and Shadowing
    // A scope always indicating the opening and cloisng curlly brackets { },
    // A scope is the range within the program for which the item is valid.

    // let x: i32 = 10;
    // {
    //     let x: i32 = 5; // it is shadowing the parent initializing and scope of this variable x = 5 is only within this block
    //     println!("value of x {} from inside block", x);
    // }

    // println!("Value of x {} from the main block", x);

    // Output
    //value of x 5 from inside block
    // Value of x 10 from the main block
    // ...........................................................................................................................................

    // unused variables
    // #[allow(unused_variables)] and this line of the top
    // let _x = 1; // or use _ prefix before initializing variable
    // if you don't want to use variable but initailized

    // ...........................................................................................................................................
    // Destructuring
    // tuple is another immutable date structure type we will explore it in latter

    // let (mut x, y) = (1, 2);
    // x += 2;

    // assert_eq!(x, 3);
    // assert_eq!(y, 2);
    // println!("Success");

    //Desturcturing Assignments
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    assert_eq!([x, y], [3, 2]);
    println!("Success");
}
