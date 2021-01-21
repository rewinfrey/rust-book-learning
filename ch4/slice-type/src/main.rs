/*

The slice type

slices let you reference a subset of elements in a collection and does not have ownership.
*/

fn main() {
    println!("Hello, world!");
}

// first_word return the index of the first space or the whole word of the given input string.
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // Convert string into a byte array.

    for (i, &item) in bytes.iter().enumerate() { // Create an interator over the array. Here, enumerate returns a tuple, the index of the item and the item itself.
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

/*

The problem with this function is that nothing preserves the index of a string in the event the string is cleared, or mutated. Instead, we can use string slices to refer to parts of the string, and only the portion of the string referenced by the slice will be used.

e.g.

let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

The range `0..5` means from the 0th element to the 4th indexed element (ending range value exclusive, not inclusive).

If we default to using the 0th index as the starting value for the range, we can omit the 0. These two statements are equivalent:

let slice = &s[0..2];
let slice = &s[..2];

This is also true for the end of the string:

let len = s.len();
let slice = &s[3..len];
let slice = &s[3..];

With that in mind, rewriting the first_word function:

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

Now that first_word returns a string slice, instead of an index pointing to where a word ends in the string, the Rust compiler knows how to make sure the reference to the slice remains valid.

e.g.

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}

Now the compiler errors:
cannot borrow `s` as mutable because it is also borrowed as immutable.

The compiler is letting us know that the string slice we borrowed from the original String is immutable,
but we're treating the String as a mutable value (by attempting to clear() it). This results in an ownership error.

String Literals are Slices

let s = "Hello, world!";

s is a compiled reference during runtime. This is why `str` are alawys immutable references, because they refer to something that is statically stored in the binary of the compiled program. &str is a reference to the string value compiled into the binary.

We can also improve the `first_word` function:

fn first_word(s: &str) -> &str {
    ...
}

By making the parameter's type `&str` (a string slice), we can immutably borrow a slice from the String, allowing this function operate on both strings and String.

This means the caller needs to pass a slice. To do this, we'd use the range syntax.

let s = String::from("hello world");

let word = first_word(&s[..]); // Pass the full String but as a slice.

let my_string_literal = "hello world";

let word = first_word(&my_string_literal[..]); // Pass the full string literal as a slice.

Slices are usually associated with strings, but they work for other collection types, like arrays, too.

let a = [1,2,3,4,5];

let slice = &a[1..3];
*/
