fn main() {
    // Exercise 1
    // Patterns
    // Use | to match several values, use ..= to match an inclusive range.
    // match_number(2);
    // match_number(0);
    // match_number(7);

    // Exercise 2
    // let p = Point {x:2, y:20};

    // match p {
    //     Point {x, y: 0} => println!("one the x axis at {}", x),
    //     Point {x:0..=5, y: y@ (10 | 20 | 20)} => println!("on the y axis at {}", y),
    //     Point {x, y} => println!("On another axis: {} {} ",x,y),
    // }

    // Exercise 3
    // @ binding operator
    // let msg = Message::Hello {id: 5};

    //  match msg {
    //     Message::Hello {
    //         id:id@ 3..=7
    //     } => println!("Found an id {}", id),
    //     Message::Hello { id: newid@(10 | 11 | 12)} =>  {
    //         println!("found id inanother range {}", newid)
    //     },
    //     Message::Hello {id} => println!("Found Some other id:{}",id),
    //  }

    // Exercise 4
    // match gaurd ( when we use if in pattren)
    // let num = Some(4);
    // let split = 5;
    // match num {
    //     Some(x) if x < split => assert!(x < split),
    //     Some(x) => assert!(x >= split),
    //     _ => (),
    // }
    // println!("Success");

    // Exercise 5
    // ..(rest pattern)
    // let number = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    // match number {
    //     (first, .., last) => {
    //         assert_eq!(first, 2);
    //         assert_eq!(last, 2048);
    //     }
    // }
    // println!("success");

    // Exercsie 6
    // &mut pattern
    let mut v = String::from("Hello,");
    let t = &mut v;
    match t {
        value => value.push_str(", World")
    }
}

enum Message {
    Hello { id: i32 },
}
struct Point {
    x: i32,
    y: i32,
}

// #[allow(unused_code)]
fn match_number(n: u32) {
    match n {
        1 => println!("One!"),
        2 | 3 | 4 | 5 => println!("2 -> 5"),
        6..=10 => println!("6 -> 10"),
        _ => println!("match -infinite -> 0 or 11 +infinite"),
    }
}
