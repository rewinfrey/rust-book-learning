// Strings store utf-8.
fn main() {
    // We can make a new empty string.
    let mut s = String::new();

    // We can push to the string.
    s.push_str("bar");

    // We can also construct a String from a str slice.
    // This is equivalent to String::from("initial contents");
    let s2 = "initial_contents".to_string();

    // push takes a single character:
    let mut t = String::from("lo");
    t.push('l');

    // Strings can be concatenated with +:
    let a = String::from("hello");
    let b = String::from("goodbye");
    let c = a + &b;

    println!("{:?}", s);
    println!("{:?}", s2);
    println!("{:?}", t);
    println!("{:?}", c);
    println!("{:?}", c);

    // For more involved string concatenation, we can use the format! macro:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    println!("{}", format!("{}-{}-{}", s1, s2, s3));

    // Rust doesn't make assumptions about what format you want to return from a string.
    // It might be characters, bytes, or scalar values.
    // Things get complicated with UTF encodings. So to avoid all that, it's up to the programmer
    // to determine what reprsentation makes the most sense when needing to slice a string or get an element at an index.
    let s4 = String::from("hello world");
    for c in s4.chars() {
        println!("{}", c);
    }
}
