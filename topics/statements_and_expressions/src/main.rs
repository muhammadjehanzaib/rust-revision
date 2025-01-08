fn main() {
    //Statements and Expressions 

    // let x = 5u32;

    // let y = {
    //     let x_square = x * x;
    //     let x_cube = x_square * x;

    //     x_cube + x_square + x 
    // };
    // let z = {
    //     // i have used semicolon in next line expressio means nothing is returning so the returning value would be ()
    //     2 * x; // it has value but we are doing nothing with it
    // };
    // println!("x is {:?}", x);
    // println!("y is {:?}", y);
    // println!("z is {:?}", z);


    // excersice 1
    // let v:u32 = {
    //     let mut x:u32 = 1;
    //     x += 2; // we can't directly return this express
    //     x // also use "return x" explicitly
    // };

    // assert_eq!(v, 3);
    // println!("Success!");

    // excersice 2

    // let v = {let x = 3; x};
    // assert!(v == 3);
    // println!("{}", v);

    // excersice 3
    let s = sum(1,2);
    assert_eq!(s,3);
    println!("Success");

}
#[allow(dead_code)]
fn sum(x:i32, y:i32) -> i32 {
    x + y
}