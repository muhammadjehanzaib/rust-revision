fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    assert_eq!(rect.area(), 1500);
    println!("Success");
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
