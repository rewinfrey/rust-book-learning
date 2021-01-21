fn main() {

    // If else
    let number = 5;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // This won't compile. Conditions must be a bool value.
    // let number = 3;
    // if number {
    //     println!("number was three");
    // }

    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }

    // If / else if
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divislbe by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    // If within a let statement
    let condition = true;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // Loops in Rust come in the form of loop, while, and for.

    // loop

    let mut counter: u32 = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break double(counter);
        }
    };

    println!("The value of result is {}", result);

    // while

    let mut number: u32 = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFT OFF!");

    // for

    let a: [u32; 5] = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Can also use a for loop for simple ranges:
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFT OFF!");

}

fn double(x: u32) -> u32 {
    x * 2
}
