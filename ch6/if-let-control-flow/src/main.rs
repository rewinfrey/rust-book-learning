/*
Concise Control Flow with `if let`

`if let` lets us match against a single pattern while ignoring others (i.e. can focus on a single variant of an enum and not handle all variants).

`if let` takes a pattern as an expression:

if let Some(3) = some_u8_value {
    println!("three");
}

What we gain in concision is lost in precision (because not all patterns are matched).

`if let` also allows us to define an else condition (like a default case):

*/

#[derive(Debug)]
enum State {
    California,
}

#[derive(Debug)]
enum Coin {
    Quarter(State),
}

fn main() {
    let coin = Coin::Quarter(State::California);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("count is {}", count);
}
