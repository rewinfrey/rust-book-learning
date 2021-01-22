/*
The match operator allows comparison against various patterns (e.g. literal values, variable names, wildcards, variants, etc.).

The compiler will also confirm that all possible cases have been written, so there isn't a runtime error if a condition is missing.

*/

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arkansas,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

/*

Matching with Option<T>

Matches must be exhuastive, so we have to match all variants (in this case Option has two variants (Some(T) and None)). Otherwise, the compiler will error.

*/
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn main() {
    println!("penny value is {}", value_in_cents(Coin::Penny));
    println!("nickel value is {}", value_in_cents(Coin::Nickel));
    println!("dime value is {}", value_in_cents(Coin::Dime));
    println!("quarter value is {}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("quarter value is {}", value_in_cents(Coin::Quarter(UsState::Alaska)));
    println!("quarter value is {}", value_in_cents(Coin::Quarter(UsState::Arkansas)));
    println!("quarter value is {}", value_in_cents(Coin::Quarter(UsState::California)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five value is {:?}", five);
    println!("six value is {:?}", six);
    println!("none value is {:?}", none);
}
