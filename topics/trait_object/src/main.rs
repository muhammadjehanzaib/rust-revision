fn main() {
    // Exercsie 1
    // let duck = Duck;
    // duck.swim();
    // let bird = hatch_a_bird(2);
    // assert_eq!(bird.quack(), "duck duck");
    // let bird = hatch_a_bird(1);
    // assert_eq!(bird.quack(), "swan swan");
    // println!("Success!");

    // Exercise 2
    // let birds: [Box<dyn Bird>; 2] = [Box::new(Duck {}), Box::new(Swan {})];

    // for bird in birds {
    //     println!("{}", bird.quack());
    // }

    // Exercise 3
    // let x = 1.1f64;
    // let y= 8u8;

    // draw_with_box(Box::new(x));
    // draw_with_ref(&y);
    // println!("Success");

    // Exercise 4
    // let x = 10u8;
    // let y = 1.33f64;

    // static_dispatch(x);
    // dynamic_dispatch(&y);

    // Exercise 5
    my_function(Box::new(8u8));
    my_function(Box::new(String::from("Abc")));
    println!("Success");
}

trait MyTrait {
    fn f(&self) -> Box<dyn MyTrait>;
}

impl MyTrait for u8 {
    fn f(&self) -> Box<dyn MyTrait> {
        Box::new(43)
    }
}

impl MyTrait for String {
    fn f(&self) -> Box<dyn MyTrait> {
        Box::new(self.clone())
    }
}

fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait> {
    x.f()
}

// trait Foo {
// fn method(&self) ->String;
// }

// impl Foo for u8 {
//     fn method(&self) -> String {
//         format!("u8: {} ", *self)
//     }
// }

// impl Foo for f64 {
//     fn method(&self) -> String {
//         format!("u64: {}", *self)
//     }
// }

// fn static_dispatch<T:Foo>(x: T) {
//    println!("{:#?}",x.method());
// }

// fn dynamic_dispatch(x: &dyn Foo) {
//     println!("{:#?}",x.method());
// }

// fn draw_with_box(x: Box<dyn Draw>) {
//     x.draw();
// }
// fn draw_with_ref(y: &dyn Draw) {
//     y.draw();
// }

// trait Draw {
//     fn draw(&self)-> String;
// }

// impl Draw for u8{
//     fn draw(&self) -> String {
//         format!("u8: {}", *self)
//     }
// }

// impl Draw for f64 {
//     fn draw(&self)-> String {
//         format!("f64: {}", *self)
//     }
// }

// // IMPLEMENT this function.
// fn hatch_a_bird(species: u8) -> Box<dyn Bird> {
//     match species {
//         1 => Box::new(Swan{}),
//         2 => Box::new(Duck {}),
//         _ => panic!(),
//     }
//}

// trait Bird {
//     fn quack(&self) -> String;
// }

// struct Duck;
// impl Duck {
//     fn swim(&self) {
//         println!("Look, the duck is swimming")
//     }
// }
// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck.. oh sorry, the swan is flying")
//     }
// }

// impl Bird for Duck {
//     fn quack(&self) -> String {
//         "duck duck".to_string()
//     }
// }

// impl Bird for Swan {
//     fn quack(&self) -> String {
//         "swan swan".to_string()
//     }
// }
