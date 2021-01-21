fn main() {

    // Scalar types are single value types.
    // These include Rust's built-in primitives like integers, floats, booleans, and strings.

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let spaces: String = "      ".to_string();
    let spaces: usize = spaces.len();
    println!("The value of spaces is: {}", spaces);

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let quotient2 = 2 / 10;

    let remainder = 43 % 5;

    println!("The value of sum is: {}", sum);
    println!("The value of difference is: {}", difference);
    println!("The value of product is: {}", product);
    println!("The value of quotient is: {}", quotient);
    println!("The value of quotient2 is: {}", quotient2);
    println!("The value of remainder is: {}", remainder);

    // Compound types contain multiple values packaged as a single type.
    // These include Rust's tuples and array primitives.

    // Tuples can contain values of mixed types.
    let tup: (i32, f64, u8) = (500, 6.3, 1);

    // These can be destructured:
    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    // Arrays require all elements to be of the same type.
    // Arrays are alays fixed in size. They are not allowed to grow dynamically.
    // Arrays allocate on the stack, not the heap (unlike tuples).
    let arr: [&str; 5] = ["Jan", "Feb", "Mar", "Apr", "May"];

    // Arrays don't have a default formatter, so the debug {:?} is used for printing.
    println!("The value of arr is: {:?}", arr);

    // When initializing an array it's possible to initialize the same element for size _n_.
    let arr = [3; 5];

    println!("The value of arr is: {:?}", arr);

    let mut num: u8 = 255;
    // Because u8 has a range from 0 to 255, in debug mode, this additional operation will raise a panic.
    // But in release mode, this will cause wrapping and not panic.
    // This wrapping behavior should not be depended on directly. Instead, it's best to use Wrapping (from the std library).
    num = num + 2;

    println!("The value of num is: {}", num);


}
