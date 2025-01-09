fn main() {
    //Functions
    //exercise 1
    // let (x,y) = (1,2);
    // let s = sum(x,y);

    // assert_eq!(s, 3);
    // println!("Success");

    // exercise 2
    // print();

    //exercise 3
    // Solve it in two ways
    // DON'T let `println!` work
    // never_retrurn();
    // println!("failed to panic");

    // Diverging functions
    // excercise 4
    // println!("Success");

    // excercise 5
    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}
#[allow(dead_code)]
fn sum(x: i32, y: i32) -> i32 {
    x + y
}

#[allow(dead_code)]
fn print() -> () {
    println!("success");
}
#[allow(dead_code)]
fn never_retrurn() -> ! {
    panic!("panic on purpose");
}
#[allow(dead_code)]
fn never_retrurn_fn() -> ! {
    // panic!();
    // unimplemented!();
    todo!();
    // we can use these macros to not return function to the caller
}
