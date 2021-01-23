/*

Defining Modules to Control Scope and Privacy

Modules are the highlest level component in a crate. They let us group related code together, and they also allow us to define paths, and what is public or private.

*/

/*

As an example, let's write a library crate that provides the functionality of a restaurant.

We'll have a back of house and front of house organization scheme.

*/

mod front_of_house {
    mod hosing {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }
}

/*

When using the `mod` keyword, we declare a module. Both a library and a binary crate implicitly create a `crate` module at the root of the crate. For this example, this means the resulting module tree looks like:front_of_house

crate
    - front_of_house
        - hosting
            - add_to_waitlist
            - seat_at_table
        - serving
            - take_order
            - serve_order
            - take_payment

`crate` is the root of the module tree, the next level is `front_of_house`, and so on. This module tree resembles a file system.
*/

fn main() {
    println!("Hello, world!");
}
