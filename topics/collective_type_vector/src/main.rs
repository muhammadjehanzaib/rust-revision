fn main() {
    // Vactor
    // Vectors are resizable arrays. Like slices,
    // their size is not known at compile time,
    // but they can grow or shrink at any time.

    // Basic Operations
    // Exercise 1
    // let arr:[u8; 3] = [1,2,3];
    // let v = Vec::from(arr);
    // is_vec(&v);

    // let v = vec![1,2,3];
    // is_vec(&v);

    // let v = vec!(1,2,3);
    // is_vec(&v);

    // let mut v1 = Vec::new();
    // v1.extend(arr);

    // assert_eq!(v, v1);
    // println!("Success");

    // Exercise 2
    // A vec can be extended with extend method

    // let mut v1 = Vec::from([1, 2, 3]);
    // v1.pop();
    // v1.push(3);
    // let mut v2 = Vec::new();
    // v2.extend(&v1);

    // assert_eq!(v1, v2);
    // println!("Success");

    // Exercsie 3
    // arr -> Vec
    // let arr = [1, 2, 3];
    // let v1 = Vec::from(arr);
    // let v2: Vec<i32> = v1.clone().into();

    // assert_eq!(v1, v2);

    // // Strinf -> Vec
    // let s = "hello".to_string();
    // let v1: Vec<u8> = s.into();

    // let s = "hello".to_string();
    // let v2 = s.into_bytes();

    // assert_eq!(v1, v2);

    // let s = "hello";
    // let v3 = Vec::from(s);

    // assert_eq!(v2, v3);

    // let v4: Vec<i32> = [0; 10].into_iter().collect();
    // assert_eq!(v4, vec![0; 10]);

    // println!("Success");

    // Exercise 4
    // let mut v = Vec::from([1, 2, 3]);
    // for i in 0..5 {
    //     println!("{:?}", v.get(i)) // Option<i32>
    // }

    // for i in 0..5 {
    //     // IMPLEMENT the code here...
    //     match v.get(i) {
    //         Some(e) => v[i] = e + 1,
    //         None => v.push(i + 2),
    //     }
    // }

    // assert_eq!(v, vec![2, 3, 4, 5, 6]);

    // for i in 0..5 {
    //     println!("{:?}", v.get(i))
    // }

    // println!("Success!");

    // Exercise 5
    // Slices
    // let mut v = vec![1, 2, 3];

    // let slice1 = &v[..];
    // // Out of bounds will cause a panic
    // // You must use `v.len` here
    // let slice2 = &v[0..v.len()];

    // assert_eq!(slice1, slice2);

    // // Slices are read only
    // // Note: slice and &Vec are different
    // let vec_ref: &mut Vec<i32> = &mut v;
    // (*vec_ref).push(4);
    // let slice3 = &v[0..4];

    // assert_eq!(slice3, &[1, 2, 3, 4]);

    // println!("Success!");

    // Exercise 6
    // Capacity
    // let mut vec = Vec::with_capacity(10);

    // // The vector contains no items, even though it has capacity for more
    // assert_eq!(vec.len(), 0);
    // assert_eq!(vec.capacity(), 10);

    // // These are all done without reallocating...
    // for i in 0..10 {
    //     vec.push(i);
    // }
    // assert_eq!(vec.len(), 10);
    // assert_eq!(vec.capacity(), 10);

    // // ...but this may make the vector reallocate
    // vec.push(11);
    // assert_eq!(vec.len(), 11);
    // assert!(vec.capacity() >= 11);

    // // Fill in an appropriate value to make the `for` done without reallocating
    // let mut vec = Vec::with_capacity(100);
    // for i in 0..100 {
    //     vec.push(i);
    // }

    // assert_eq!(vec.len(), 100);
    // assert_eq!(vec.capacity(), 100);

    // println!("Success!");

    // Exercise 7
    // FILL in the blank
    // let v: Vec<IpAddr> = vec![
    //     IpAddr::V4("127.0.0.1".to_string()),
    //     IpAddr::V6("::1".to_string()),
    // ];

    // // Comparing two enums need to derive the PartialEq trait
    // assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    // assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    // println!("Success!");

    // Exercsie 8
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}

trait IpAddr {
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}", self.0)
    }
}
struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}", self.0)
    }
}

// // #[derive(Debug, PartialEq)]
// // enum IpAddr {
// //     V4(String),
// //     V6(String),
// }

// fn is_vec(x: &Vec<u8>) {}
