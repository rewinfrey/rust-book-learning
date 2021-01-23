/*

Separating Modules into Different Files

Continuing with the restaurant example, let's say we want to move the front_of_house module into its own file under `src/front_of_house.rs`:

// src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}

// src/lib.rs
mod front_of_house; // Semicolon at end of this `mod` statement bring the `lib/front_of_house.rs` module into scope.

pub use crate::front_of_house::hosting; // Declare what we want to use from the module tree.

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // Call the public method `add_to_waitlist` defined in `lib/front_of_house.rs`
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

If we wanted to make `hosting` its own file, we can make `front_of_house` a directory:

// src/front_of_house.rs
pub mod hosting;

// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}

// src/lib.rs (no changes needed)
mod front_of_house;

pub use crate::front_of_house::hosting;


*/

fn main() {
    println!("Hello, world!");
}
