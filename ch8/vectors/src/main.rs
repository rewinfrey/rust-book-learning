/*

Vectors hold 0 or more values in contiguous memory (in the heap), and are one of the core collection types offered by Rust's Prelude.

Vectors are generic, and the generic type parameter is optional if the context allows for easy reference.

*/

fn main() {
    // Can use the vec! macro to instantiate a new vector.
    let mut v = vec![1,2,3];

    // Can also use the Vec path to instantiate a new vector.
    let v2: Vec<i32> = Vec::new();

    // Vectors are like stacks, we can push to them:
    v.push(4);
    v.push(5);

    // We can read elements from the vector. Note that reading an element from a vector implicitly borrows:
    let third: &i32 = &v[2];

    // Accessing an element index in a vector that doesn't exist causes a runtime panic.
    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 100', src/main.rs:24:29
    // let _hundredth: &i32 = &v[100];

    // Combine with a match clause:
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    println!("{:?}", third);

    println!("{:?}", v);
    println!("{:?}", v2);

    // Iterating over vectors with a mutable reference:
    let mut z = vec![1,2,3,4,5];
    for i in &mut z {
        *i += 50;
    }
    println!("{:?}", z);

    // Iterating over vectors with an immutable reference:
    for i in &z {
        println!("the value is {}", i);
    }

    // Using an enum to store multiple variants of the same type:
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // Vectors can only store elements of the same type. To fake this, we can use multiple variants to encode different values while still giving those values the same enum type. This is the same trick used in Haskell.
    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Float(0.01),
        SpreadsheetCell::Text(String::from("text")),
    ];

    println!("{:?}", row);
}
