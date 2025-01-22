fn main() {
    // Exercise 1
    // Struct
    // The types of structs
    // #[derive(Debug)]
    // struct Person {
    //     name: String,
    //     age: u8,
    //     hobby: String,
    // }
    // let age: u8 = 30;
    // let p: Person = Person {
    //     name: String::from("Alice"),
    //     // age: age, correct but we can also use this
    //     age, // called "field init short hand"  it is same as
    //     // It's similar to modern JavaScript object shorthand property names.
    //     //The compiler expands age to age: age automatically.
    //     hobby: String::from("Blockchain Dev"),
    // };
    // println!("{:?}", p);
    // println!("Success");

    // Exercise 2
    // let u = Unit;
    // do_something(u);
    // println!("Success");

    // Exercise 3
    // Tuple struct looks similar to tuple, it has added meaning the struct name provides
    // let v: Point = Point(0,127,255);
    // check_color(v);
    // println!("Success");

    // Exercise 4
    // Operating on Sturct
    // You can make a whole struct mutable when instantiating it, but Rust doesn't alloew us to make only certain field as mutable
    // let age = 30;
    // let mut p = Person {
    //     name: String::from("Sunface"),
    //     age,
    // };

    // p.age = 20;
    // p.name = String::from("flying Curve");
    // println!("Success");

    // Exercise 5
    // let _p = build_person(String::from("hello"), 9);
    // println!("Success");

    // Exercise 6
    // let u1 = User {
    //     email: String::from("hello@gmail.com"),
    //     username: String::from("Alice"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // let u2 = set_email(u1);
    // println!("success");

    // Exercise 7
    // Print the Sturct
    // We can use #[derive(debug)] to make sturct printable
    // let scale = 2;
    // let rect = Rectangle {
    //     width: dbg!(30 * scale),
    //     height: 50,
    // };

    // dbg!(&rect);
    // println!("{:#?}", rect);

    // Exercise 8
    // Partial Move
    let f = File {
        name: String::from("readme.md"),
        data: "Rust by practiec".to_string(),
    };

    let _name = f.name; // ownership of f.name has bee moved
    println!("{:?}, {:?}", _name, f.data); // {:?} -> :? (Debug notation)
}
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}

// Exercise 6
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn set_email(u: User) -> User {
//     User {
//         email: String::from("contrat@in.dev"),
//         ..u
//     }
// }

// Exercise 7
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// Exercise 5
// struct Person {
//     name: String,
//     age: u8,
// }

// fn build_person(name:String, age:u8) -> Person {
//     Person{
//         name,
//         age,
//     }
// }

// Exercise 4
// struct Person {
//     name: String,
//     age: u8,
// }

//Exercise 3
// struct Point(i32,i32,i32);
// struct Color(i32,i32,i32);

// fn check_color(p: Point) {
//     // INCORRECT - Can't destructure struct like a tuple
//     // let Point(x,y,z) = p; // Error
//     let Point(ref x,ref _y,ref z) = p;  //
//     // let (x,y,z) = (p.0, p.1, p.2);
//     assert_eq!(*x, 0); //  Dereferencing here
//     assert_eq!(p.1, 127);
//     assert_eq!(*z, 255);
// }

// Exercise 2
// struct Unit;
// trait SomeTrait {
//     // .... Some behaviours defined here ...
// }
// impl SomeTrait for Unit {}
// fn do_something(_u: Unit) {}
