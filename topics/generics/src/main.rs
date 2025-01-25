fn main() {
    // Generics are the palce holders for concrete types
    // Enable writing more reusable and flexible code
    // Avoid having duplicate code for different types
    // Zero cost abstraction, Rust compiler will at compile time full out generics with concrete types

    // Generics
    // Functions
    // Exercise 1
    // reg_fn(S(A));
    // gen_spec_t(SGen(A));
    // gen_spec_i32(SGen(8));
    // // Explicitly
    // generics::<char>(SGen('A'));
    // // Implicitly
    // generics(SGen('B'));
    // println!("Success");

    // turbo fish syntax
    // Exercise 2
    // assert_eq!(5, sum(2i8, 3i8));
    // assert_eq!(50, sum(20,30));
    // assert_eq!(2.46, sum(1.23, 1.23));
    // println!("Success");

    // Stuct and impl
    // Exercise 3
    // let interger = Point{x:5, y:9};
    // let float = Point{x: 1.0, y: 4.0};
    // println!("Success");

    // Exercise 4
    // let p = Point {
    //     x: 5,
    //     y: "hello".to_string(),
    // };
    // println!("Success");

    // Exercise 5
    // let x = Val { val: 3.0 };
    // let y = Val {
    //     val: String::from("Hello, it's me"),
    // };
    // println!("{} , {}", x.value(), y.value());

    // Exercise 6
    // methods
    // let p1 = Point { x: 5, y: 10 };
    // let p2 = Point {
    //     x: "Hello",
    //     y: '中',
    // };

    // let p3 = p1.mixup(p2);

    // assert_eq!(p3.x, 5);
    // assert_eq!(p3.y, '中');
    // println!("Success");

    // Exercise 7
    let p = Point { x: 5.0, y: 10.0 };
    println!("{}", p.distance_form_origin());
}

struct Point<T> {
    x: T,
    y: T,
}

impl Point<f64> {
    fn distance_form_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, others: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: others.y,
//         }
//     }
// }

// struct Val<T> {
//     val: T,
// }

// impl<T> Val<T> {
//     fn value(&self) -> &T {
//         &self.val
//     }
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
//     x + y
// }

// struct A;
// struct S(A);
// struct SGen<T>(T);

// fn reg_fn(_s: S) {}
// fn gen_spec_t(_s: SGen<A>) {}
// fn gen_spec_i32(_s: SGen<i32>){}
// fn generics<T>(_s:SGen<T>) {}
