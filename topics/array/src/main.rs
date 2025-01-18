fn main() {
    // Exercise 1
    // let arr = [1,2,3,4,5];
    // assert_eq!(arr.len(), 5);
    // println!("Success");

    // Exercise 2
    // let _arr0 = [1,2,3];
    // let arr: [char; 3] = ['a','b','c'];
    // assert_eq!(std::mem::size_of_val(&arr), 12); 
    // println!("Success");

    // Exercise 3
    // All elements in array can be initialized to the same value at once.
    //  let list: [i32; 100] = [1; 100];

    //  assert!(list[0] == 1);
    //  assert!(list.len() == 100);
    //  println!("Success");

    // Exercise 4
    // All elements must be the same type
    // let _arr = [1,2,3];
    // println!("Success");

    // Exercise 5
    // let arr = ['a', 'b','c'];
    // let ele = arr[0];

    // assert!(ele == 'a');
    // println!("Success");

    // Exercise 6
    // Out of bound indexing cause panic;
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    let _name0 = names.get(0).unwrap();

    let _name1 = &names[1];
    println!("Success");
}
