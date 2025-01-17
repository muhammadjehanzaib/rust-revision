fn main() {
    // String
    // Str and &str
    // The type of string literal "hello, world" is &str, e.g let s:&str = "hello, world".
    // 🌟 We can't use str type in normal ways, but we can use &str.
    // Exercise1
    // let _s: &str = "Hello world";
    // println!("Success");

    // Exercise 2
    // We can only use str by boxing it, & can be used to convert Box<str> to &str
    // let s:Box<str> = "hello, world".into();
    // greeting(&s);

    // Exercise 3
    // let mut s: String = "".to_string();
    // let mut s = String::from("");
    // s.push_str("Hello, world");
    // s.push('!');

    // assert_eq!(s, "Hello, world!");
    // println!("Success");

    // Exercise 4
    // let mut s = String::from("hello");
    // s.push(',');
    // s.push_str(" world");
    // s += "!";
    // println!("{}", s);

    // Exercise 5
    // let s = String::from("I like dogs");
    // let s1 = s.replace("dogs", "cats");
    // assert_eq!(s1, "I like cats");
    // assert_eq!(s, "I like dogs");
    // println!("Success");

    // Exercise 6
    // let s1 = String::from("hello,");
    // let s2: &str = &String::from("world");
    // let s3 = s1 + s2;
    // assert_eq!(s4, "hello,world");
    // println!("Success");

    // Exercise 7
    // &str and String
    // let s = "Hello, world";
    // // greeting(s.to_string()); // or s.to_owned()
    // greeting(String::from(s));

    // Exercise 8
    // let s = "hello, world".to_string();
    // // let s1: &str = &s;
    // let _s1: &str = s.as_str();
    // println!("Success");

    // Exercise 9
    // String Escapes
    // let byte_escape = "I'm writing Ru\x73\x74!";
    // println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // Exercise 10
    // let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    // println!("{}", raw_str);

    // Exercise 11
    // let s1 = String::from("hi,中国");
    // let h = &s1[0..1];
    // assert_eq!(h,"h");

    // let h1 = &s1[3..6]; // 中 takes 3 bytes that why we added 3 to 6
    // println!("{}", h1);

    // Exercise 12
    for c in "你好，世界".chars() {
        println!("{}", c);
    }

}

// #[allow(dead_code)]
// fn greeting(s: &str) {
//     println!("{}", s);
// }
#[allow(dead_code)]
fn greeting(s: String) {
    println!("{}", s);
}
