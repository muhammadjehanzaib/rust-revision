fn main() {
    // Exercise 1
    // let array = [
    //     Array{
    //         data: [1,2,3]
    //     },
    //     Array{
    //         data: [3,4,5]
    //     },
    //     Array{
    //         data: [6,7,8]
    //     },
    // ];

    // Exercsie 2
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", ",world"];
    print_array(arr);
}

fn print_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

struct Array<T, const N: usize> {
    data: [T; N],
}
