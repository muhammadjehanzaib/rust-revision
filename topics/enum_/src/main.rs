fn main() {
    // Exercise 1
    // Enums can be created with explicit discrimination
    // assert_eq!(Number::One as u8, Number1::One as u8);
    // assert_eq!(Number1::One as u8, Number2::One as u8);

    // Exercise 2
    // let msg1 = Message::Move { x: 1, y: 2 };
    // let msg2 = Message::Write(String::from("Hello, World"));

    // println!("Success");

    // Excercise 3
    // We can get the data which an enum variant is holding by patteren match.
    // let msg = Message::Move{x: 1, y: 1};
    // if let Message::Move{x:a, y:b} = msg {
    //     assert_eq!(a,b);
    // }else {
    //     panic!("Never let this run");
    // }
    // println!("Success");
    // OR
    // match msg {
    //     Message::Move{x:a, y:b} => {
    //         assert_eq!(a,b);
    //     },
    //     _ => panic!("Never let this run")
    // }
    // println!("Success");

    // Exercise 4
    // let msgs: [Message;3] = [
    //     Message::Quit,
    //     Message::Move{x:1, y:2},
    //     Message::ChangeColor(255,255,0)
    // ];

    // for msg in msgs {
    //     show_message(msg);
    // }

    // Exercise 5
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let Some(n) = six {
        println!("{}", n);
        println!("Success");
    } else {
        panic!("Never let me run this");
    }


}

// Exercise 5
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(v) => Some(v + 1),
    }
}

// fn show_message(msg: Message) {
//     println!("{:?}", msg);
// }
// Exercise 2
// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
// // Exercise 1
// enum Number {
//     Zero,
//     One,
//     Two,
// }

// enum Number1 {
//     Zero = 0,
//     One,
//     Two,
// }
// enum Number2 {
//     Zero = 0,
//     One = 1,
//     Two = 2,
// }
