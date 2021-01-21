/*

Rust enums are most similar to algebraic data types in languages like F#, OCaml, and Haskell.

Together with the match keyword, enums make it possible to express multiple values of the same type, and handle those values differently.

*/

/*
Because ip addresses can either conform to ipv4 or ipv6, but not both, an enum encodes this choice without allowing an ip address to be expressed as both. Additionally, wherever our code uses the IpAddrKind type, we can conditionally handle v4 or v6 is necessary. Otherwise, we can treat all IpAddrKind values in the same way.

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

We could use a struct to further encode the address data associated with an IpAddrKind. But there is a shorter, more concise way to do this using the enum type directly:

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};

Why enums over structs? If there are multiple variants that are to be treated like a single type, enums also make it possible to define methods on that type (with the help of an impl block). Doing this for structs would require separate method definitions for each struct.
*/

/*
Enums also allow us to encode additional data into them directly.
*/
#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

/*
Each variant of an enum can contain different data.
*/
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    route(home);
    route(loopback);

    let home2 = IpAddr::V4(127, 0, 0, 1);
    let loopback2 = IpAddr::V6(String::from("::1"));

    route2(home2);
    route2(loopback2);
}

fn route(ip_kind: IpAddrKind) {
    println!("routing {:?}", ip_kind);
}

fn route2(ip_kind: IpAddr) {
    println!("routing {:?}", ip_kind);
}
