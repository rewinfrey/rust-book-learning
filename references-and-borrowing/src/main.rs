/*

References in Rust allow values to be passed to different contexts without that context taking ownership.

* At any given time, you can have either one mutable reference, or multiple immutable references to a single value.
* References must always be valid (the compiler gives us some guarantees).

The opposite of a reference is a dereference (using the * symbol).

Creating references to values does not move ownership. The value pointed to by the reference is not dropped when the reference goes out of scope.

This behavior is called borrowing in Rust. Even though references do not move ownership, it does not mean that concurrent modification of a borrowed value is allowed.

e.g.

fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}

This fails to compile b/c some_string is a borrowed reference and is not mutable. Only owners can mutate their values.

Rust allows something called a mutable reference, however:

e.g.

fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

Because everywhere the reference is now used also includes the `mut` keyword, this reference is borrowed and allowed to mutate the underlying value. However, only one mutable reference is allowed within a single scope at a time.

e.g.

fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}

This code fails to compile because r1 is a mutable reference, but r2 is attempting to also be a mutable reference of the same value within the same scope. The compiler error reads:

"cannot borrow `s` as mutable more than once at a time"

This restriction eliminates the possibility of common data races, at compile time.

A data race occurs whenever:

* Two or more pointers access the same data at the same time.
* At least one of the pointers is being used to write to the data.
* There's no mechanism being used to synchronize access to the data.

We can always introduce a new scope to allow for multiple mutable refs to a single value:

e.g.

let mut s = String::from("hello");

{
    let r1 = &mut s; // this is allowed because the {} create a new scope, and r1 is drop'ed when this scope closes
}

let r2 = &mut s;

It's also not possible to mix and match mutable and immutable refs to the same value:

let mut s = String::from("hello");

let r1 = &s;     // ok
let r2 = &s;     // ok
let r3 = &mut s; // error

The compiler here tells us: "cannot borrow `s` as mutable because it is borrowed as immutable".

This is because when an immutable reference exists within the same scope as a mutable reference, then the reader of the immutable reference can trigger a data race with the owner of the mutable ref. Therefore, this condition is not allowed.

Rust avoids the problem of dangling pointers (a pointer to a location in memory that may have moved or been freed). Even if we try, Rust will ensure that a value will not be freed so long as a reference to the value is still within scope.

e.g.

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s // return a reference to s
} // s goes out of scope and is dropped; causing an error

The compiler tells us: "missing lifetime specifier"
"this function's return type contains a borrowed value, but there is no value for it to be borrowed from."

This is because the dangle() function returns a reference to a String, but the reference to the String goes out of scope when dangle() returns. This means we're attempting to return a value that has gone out of scope, which is not possible. Because `s` is created within `dangle()`, we need to make sure that ownership of `s` is moved when the function returns.

fn dangle() -> String {
    let s = String::from("hello");
    s
}

This makes the compiler happy. Now `s` is created with `dangle()`, but instead of returning a reference to `s`, `s` itself is returned directly which moves ownership of `s` to the caller.

*/

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // The & indicates passing a reference in this function call.

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
