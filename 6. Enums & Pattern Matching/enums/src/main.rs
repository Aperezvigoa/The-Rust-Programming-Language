// Enums
// Enums allow us to define a type by enumerating its possible variants.

// Defining an Enum
// Where structs give you a way of grouping together related fields and data, enums give you
// a way of saying a value is one of a possible set of values.For example, we may want to
// say that Rectangle is one of a set of possible shapse that also includes Circle and
// Triangle. To do this, Rust allows us to encode these possibilities as an enum.

// Lets look at a situation we might want to express in code and see why enums are useful
// and more appropiate than structs in this case. Say we need to work with IP addresses.
// Currently, two maajor standdards are used for IP addresses: version four and version six.
// Because these are the only possibilities for an IP address that our program will come
// across, we can enumerate all possible variants, which is where enumeration gets it name.

// Any IP address can be either a version four or a version six address, but not both at the
// same time. That property of IP addresses makes the enum data structure appropiate because
// an enum value can only be one of its variants. Both version four and version six address
// are still fundamentally IP addresses, so they should be trated as the same type when the
// code is handling situations that apply to any kind of IP address.

// We can express this concept in code by defining an IpAddrKind enumeration and listing the
// possibble kinds an IP adress can be, V4 and V6:

enum IpAddrKind {
    V4,
    V6,
}
// IpAddrKind is now a custom data type that we can use elsewhere in our code.

// Using enums has even more advantages. Thinking more about our IP addrss type, at the moment
// we dint have a way to store the actual IP address data; we only know what kind it is.

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Direction {
    north(String),
    south(String),
}

enum AllInOneIpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// This enum has four variants with different types:
// Quit -> No data associated
// Move -> Has named fields, like a struct
// Write -> Includes a single String,
// ChangeColor -> Includes three i32

// We can define methods on enums as we do with structs

impl Message {
    fn call(&self) {
        // method body
    }
}

// The Option Enum and Its Advantages Over Null Values

// This section explores a case study of Option, which is another enum defined by the standard
// library. The Option type encodes the very common scenario in which a value could be
// something or it could be nothing.
// For example, if you request the first item in a non-empty list, you would get a value. If
// you request the first item in an empty list, you would get nothing. Expressing this concept
// in terms of the type system means the compiler can check wether you've handled all the cases
// you should be handling; this functionality can prevent bugs that are extremely common in
// other programming languages.
// Programming language design is often though of in term of which features you include, but the
// features you exluce are important too. Rust doesnt have the null feature that many other
// languages have. Null is a value that means there is no value there. In languages with null,
// variables can always be in one of two states: null or not-null.
// The problems with null values is that if you try to use a null value as a not-null value, you
// ll get an error of some kind. However, the concept that null is trying to express is still a
// useful one: A null is a value that is currently invalid or absent for some reason.
// The problem isn't really with the concept but with the particular implementation. As such, Rust
// does not have nulls, but it does have an enum that can encode the concept of a value being
// present or absent. This enum is Option<T>

enum Option<T> {
    None,
    Some(T),
}

// The Option<T> enum is so useful that its even included in the prelude; you don't need to bring
// it into scope explicitly. Its variants are also included in the prelude: you can use Some and
// None directly without the Option:: prefix. The Option<T> enum is still just a regular enum, and
// Some(T) and None are still variants of type Option<T>.
// The <T> syntax is a feaure of Rust we havent seen yet. It's a generic type parameter, and we'll
// cover generics in more detail. For now, we have to know that <T> means that the Some variant of
// the Option enum can hold one piece of data of any type, and that each concrete type that gets
// used in place of T makes the overall Option<T> type a different type.

fn main() {
    // Enum Values
    // We can create unstances if each of two variants of IpAddrKing like this:
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    // Four and six, are of the same type: IpAddrKind. We can then define a function than takes
    // any IpAddrKind.
    route(four);
    route(six);

    // Using enums & structs.
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let north_pole = Direction::north(String::from("Here is Santa Clous"));

    let another_home = AllInOneIpAddr::V4(127, 0, 0, 1);
    let another_loopback = AllInOneIpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello Rust!"));
    m.call();

    // Using Option<T> enum
    let some_number = Some(5);
    let some_char = Some('a');

    let abstent_number: Option<i32> = None;
    // Option eliminates the risk of incorrectly assuming a not-null value, we can't add for example
    // an i8 with a Option<i8>, we need to convert Option<T> to T.
}

fn route(ip_kind: IpAddrKind) {
    println!("This is a IpAddrKind function");
}
