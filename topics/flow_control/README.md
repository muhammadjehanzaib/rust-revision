# Rust Flow Control Exercises

## Exercise 1
```rust
let n = 5;

if n < 0 {
    println!("{} is negative", n);
} else if n > 0 {
    println!("{} is positive", n);
} else {
    println!("{} is zero", n);
}
```

## Exercise 2
🌟🌟 **If/else expression can be used in assignments.**

```rust
let n = 5;

let big_n = if n < 10 && n > -10 {
        println!(", and is a small number, increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, halve the number");
        n / 2
    };

println!("{} -> {}", n, big_n);
```

## Exercise 3
```rust
for n in 1..100 {
    if n == 100 {
        panic!("Never let this run");
    }
}
println!("Success");
```

## Exercise 4
```rust
let names = [String::from("liming"), String::from("hanmeimei")];
for name in &names {
    // Do something with name...
    println!("{:?}", name);
}

println!("{:?}", names);

let numbers = [1, 2, 3];
// The elements in numbers are Copy，so there is no move here
for n in numbers {
    // Do something with n...
}

println!("{:?}", numbers);
```

## Exercise 5
```rust
let a = [5, 4, 3, 2, 1];
for (i, v) in a.iter().enumerate() {
    println!("{} {}", i + 1, v);
}

for i in 0..a.len() {
    println!("{} {}", i + 1, a[i]);
}
```

## Exercise 6
**While Loop Example**

```rust
let mut n = 1;

// Loop while the condition is true
while n < 10 {
    if n % 15 == 0 {
        println!("fizzbuzz");
    } else if n % 3 == 0 {
        println!("fizz");
    } else if n % 5 == 0 {
        println!("buzz");
    } else {
        println!("{}", n);
    }

    n += 1;
}

println!("n reached {}, so loop is over", n);
```

## Exercise 7
**Using `break` to Exit a Loop**

```rust
let mut n = 0;
for _i in 0..=100 {
    if n == 66 {
        break;
    }
    n += 1;
}
assert_eq!(n, 66);
println!("Success");
```

## Exercise 8
**Using `continue`**

```rust
let mut n = 0;
for i in 0..=100 {
   if n != 66 {
       n += 1;
       continue;
    }
    break;
}

assert_eq!(n, 66);

println!("Success!");
```

## Exercise 9
**Infinite Loop with `loop`**

```rust
let mut count = 0u32;

println!("Let's count until infinity!");

// Infinite loop
loop {
    count += 1;

    if count == 3 {
        println!("three");

        // Skip the rest of this iteration
        continue;
    }

    println!("{}", count);

    if count == 5 {
        println!("OK, that's enough");

        break;
    }
}

assert_eq!(count, 5);

println!("Success!");
```

## Exercise 10
🌟🌟 **Using `loop` with `break` to Return a Value**

```rust
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 20 {
        break counter;
    }
};

assert_eq!(result, 20);

println!("Success!");
```

## Exercise 11
**Nested Loops with Labels**

```rust
let mut count = 0;
'outer: loop {
    'inner1: loop {
        if count >= 20 {
            // This would break only the inner1 loop
            break 'inner1; // `break` is also works.
        }
        count += 2;
    }

    count += 5;

    'inner2: loop {
        if count >= 30 {
            // This breaks the outer loop
            break 'outer;
        }

        // This will continue the outer loop
        continue 'outer;
    }
}

assert!(count == 30);

println!("Success!");
```