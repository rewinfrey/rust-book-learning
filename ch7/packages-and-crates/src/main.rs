/*
Packages and Crates

crate: a binary or library. By convention, a binary crate's root is `src/main.rs`, a library crate's root is `src/lib.rs`.
package: one or more crates defined by a Cargo.toml file. Created with `cargo new`.

packages can have zero or one library crates, but no more than one.

Cargo passes the crate root to `rustc` at compile time, and walks the module / files / paths to compile the needed code.

For multiple binary crates in a single package, place separate binary crates within `src/bin` directory. Each file will be a separate binary crate.

*/

fn main() {
    println!("Hello, world!");
}
