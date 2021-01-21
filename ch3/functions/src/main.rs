fn main() {
    println!("Hello, world!");

    let y = {
        let x = 4;
        // There is no semicolon at the end of this expression. Doing so turns this expression into a statement, which will result in no value being assigned to `y`.
        x + 1
    };

    another_function(5, y);

    println!("The value of five is: {}", five());
    println!("The value of plus_one(five()) is: {}", plus_one(five()));
}

fn another_function(x: u32, y: u32) {
    println!("Another function.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> u32 {
    5
}

fn plus_one(x: u32) -> u32 {
    x + 1
}
