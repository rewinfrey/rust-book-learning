/*

Bringing Paths into Scope with the `use` Keyword

Rather than only refer to items via its absolute or relative path in the module tree, we can simplify the path used by using the `use` keyword.

*/

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Absolute path
// use crate::front_of_house::hosting;

// Relative path
// use self::front_of_house::hosting; // this is the preferred way

// More specific path
use self::front_of_house::hosting::add_to_waitlist; // this is too granular, usually

pub fn eat_at_restaurant() {
    // hosting::add_to_waitlist();
    // hosting::add_to_waitlist();
    // hosting::add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}

// Sometimes we may want to bring items into scope with the same name:
// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {}
// fn function2() -> io::Result<()> {}

// But if we had used instead:
// use std::fmt::Result;
// use std::io::Result;

// Then we'd have a problem if we were only using `Result` as the identifier.

// We can alias items in the module tree:
// use std::io::Result as IoResult;
// fn function2() -> IoResult<()> {}

// We can also make items we `use` public in the module tree when they are used:
// pub use crate::front_of_house::hosting;

/*
Nested paths

Instead of writing:

use std::cmp::Ordering;
use std::io;

We can nest the paths:

use std::{cmp::Ordering, io};

Another example with `self`:

use std::io;
use std::io::Write;

Becomes with `self`:
use std::io::{self, Write};

*/

/*
Glob operator

If we want to bring all public items in a module tree into scope, we can use the glob operator:

use std::collections::*;
*/

fn main() {
    eat_at_restaurant();
}
