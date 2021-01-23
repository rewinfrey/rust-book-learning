/*

Paths for Referring to an Item in the Module Tree

To locate an item in a module tree, we need to refer to that item via its path throught the module tree.

absolute path: starts from the crate root by using a crate name or a literal `crate`.
relative path: starts from the current module and uses `self`, `super`, or an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers separated by `::`.

*/

/*

Let's use the example of the restaurant:

*/


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn serve_order() {}

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        // The use of `super` here looks to the parent module in the module tree.
        // In this instance, that means the parent module to `back_of_house`, which is the current module.
        super::serve_order();
    }

    pub fn cook_order() {}

    // We can also define structs and enums as public:
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!("Order 1 is {:?}", order1);
    println!("Order 2 is {:?}", order2);
}

pub fn fix_incorrect_order() {
    // Absolute path
    crate::back_of_house::fix_incorrect_order();

    // Relative path
    back_of_house::fix_incorrect_order();
}

fn main() {
    eat_at_restaurant();
}
