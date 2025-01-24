fn main() {
    // Exercise 1
    // let rect = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    // assert_eq!(rect.area(), 1500);
    // println!("Success");

    // Exercise 2
    // let light = TrafficLight{
    //     color: String::from("Red"),
    // };

    // light.show_state();
    // println!("{:?}", light);

    // Exercise 3
    // in this exercise we explore diff btw &self and &mut self

    // Exercise 4
    // Associated Functions
    // let light = TrafficLight::new();
    // assert_eq!(light.get_state(), "red");
    // println!("Success");

    // Exercise 5
    // Multiple impl blocks
    // println!("Success");

    // Exercise 6
    // Enum
    // We can also implement methods for enums
    let light = TrafficLightColor::new();
    // assert_eq!(light, TrafficLightColor::Yellow);
    println!("{:?}", light);
}

#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Green,
    Yellow,
}

impl TrafficLightColor {
    pub fn new() -> Self {
        Self::Yellow
    }
}


#[derive(Debug)]
struct TrafficLight {
    color: String,
}
// Exercise 4
impl TrafficLight {
    pub fn new() -> Self {
        Self {
            color: String::from("red"),
        }
    }
    pub fn get_state(&self) -> &str {
        &self.color
    }
}

// Exercise 2 and 4
// impl TrafficLight {
//     pub fn show_state(self: &Self) {
//         // these arrguments are the same as (&self)
//         println!("Current state of signal is :{}", self.color);
//     }
//     pub fn change_state(&mut self) {
//         self.color = "green".to_string();
//     }
// }

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
        
    }
}