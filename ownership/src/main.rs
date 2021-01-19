/*

Ownership in Rust

Key things to remember:

1) Each value in Rust has a variable that's called its owner.
2) There can only be one owner at a time.
3) When the owner goes out of scope, values assocated with the owner are dropped.

Any value that is fixed in size will be placed on the stack.

Function arguments are always placed on the stack (but for dynamically sized collections or values, the pointer is placed on the stack). All other values are stored on the heap.

Stack is more efficient and faster for lookups. Heap is more flexible and allows for dynamic allocations.

For heap allocated values, e.g.:

let s1 = String::from("hello");
let s2 = s1;

s1 is said to "move" to s2, indicating that the value stored by s1 only has a single owner at a time.

But for stack allocated variables, e.g.:

let x = 5
let y = x

x is said to be "clone"ed to y, and both instances of this value are valid. Litearls that are not dynamically allocated are cloned rather than moved. This behavior is provided by the Clone trait.

Rust doesn't let us mix and match the `Drop` and `Clone` traits. A type can only implement one or the other, otherwise Rust will throw a compile time error.

e.g.
fn main() {
    let s = String::from("hello");

    takes_ownership(s)

    println!("{}", s) // this fails to compile because ownership of s has been moved.

    let x = 5;

    makes_copy(x);

    println!("{}", x) // this compiles b/c x was copied when `makes_copy(x)` was called.
}

In the example above, when a heap-allocated value is passed as an argument to a function, the function takes ownership of the value. But when a stack-allocated value is passed as an argument and that value's type is annotated with the Copy trait, the value is copied leaving the original value alone (with the same ownership) while the new value is owned only for the lifetime of the function call.

But it is possible to return values, which also returns ownership for heap-allocated values.

e.g.
fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
*/

/*

Heap allocation vs Stack allocation

Pushing to the stack is usually faster. Allocating to the heap is usually slower.

Heap allocations requrie finding contiguous memory. This usually requires allocating a chunk of memory.

Stack allocations are speedy because they are pushed onto the stack. There is no searching for a contiguous chunk of memory, that is also required to be allocated before values can be written to it.

In Rust, the String type is allocated on the heap (indicating String values are dynamically sized).

A string value is a struct with three fields:

* ptr: points to the value in memory (stored in the heap)
* len: length of current value in bytes in memory
* capacity: total number of bytes received from allocator

*/

/*

When a variable goes "out of scope", Rust calls the `drop` method on it, and comes from the RAII (resource acquisition is initialization) pattern common to C++.

*/


fn main() {
    let mut s = String::from("hello");
    s.push_str(", world.");

    /*
        This fail to compile because `s` becomes invalid after it moves to `j`.
        This helps to prevent the "double free" error.

        If this relied on a simple, shallow copy alone, the data pointed to by the String instance would be shared,
        because Rust `drop`s when a scope exits, both instances would `drop` the same index in memory, causing data corruption.

        let j = s;
        println!("The value of s is: {}", s);
    */

    s.push_str(" It's a great day to be here.");

    println!("The value of s is: {}", s);

    /*

        Rust does not automatically perform "deep" copies of data structures. That allocation is always an explicit step.

        `clone()`

        The clone operation is a copy of heap data. Remember, heap data has two penalties:
            1) accessing heap data is always slower than accessing stack data
            2) writing heap data is always slower than writing stack data

        Automatic copies (aka "shallow" clones -- but the Rust version of "shallow" clone which invalidates the source reference, leaving only the target reference as the still valid reference to the heap data), are relatively inexpensive.

        This is a simple example of how to clone a String.
    */

    let mut s2 = s.clone();

    s2.push_str(" Aloha friends!");

    println!("The value of s2 is: {}", s2);
}
