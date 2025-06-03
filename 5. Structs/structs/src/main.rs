// **** Using Structs to Stucture Related Data ****
// A struct, or structure, is a custom data type that lets you package together
// and name multiple related values that make up a meaningful group.A struct is
// like an object's data attributes.

// **** Defining and Instantiating Structs ****
// Structs are similar to tuples. Like tuples, the pieces of a struct can be
// different types. Unlike tuples, in a struct you'll name each piece of data
// so it's clear what the values you mean. Adding these names means that structs
// are more flexible than tuples: you dont have to rely on the order of the data
// to specify or access the value of an instance.
// To define a struct, we enter the keyword struct and name the entire struct. A
// struct's name should describe the significance of the pieces of data being
// grouped together.

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple struct:
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like struct:
struct AlwaysEqual;

// To use a struct after we've definded it, we create an instance of that struct
// by specifying concrete values for each of the fields.

fn main() {
    let user1 = User {
        active: true,
        username: String::from("Jon Doe"),
        email: String::from("jondoe@doejon.jon"),
        sign_in_count: 69,
    };
    // To access a specific value from a struct, we use dot notation.
    let user_email: String = user1.email;
    println!("The user mail is {user_email}");

    // If the instance is mutable we can change data using the same notation.
    let mut user2 = User {
        active: false,
        username: String::from("Olivia"),
        email: String::from("olivialovesjon@gmail.com"),
        sign_in_count: 3,
    };
    user2.email = String::from("oliviahatesjon@gmail.com");

    // All the entire instances must be mutable, Rust doesnt allow us to mark only
    // certain fields as mutable.

    // - Creating Instances from Other Instances with Struc Update Syntax
    let user3 = User {
        active: user1.active,
        username: user2.username,
        email: user2.email,
        sign_in_count: user2.sign_in_count,
    };
    // This code syntax moves the ownership of username and email, so user2 Strings are
    // now invalid, they lose their ownership.

    // Using Tuple Sctructs Without Named Fields to Create Different Types
    // Rust also supports strucst that look similar to tuples, tuple structs. Tuple structs
    // have the added meaning the struct name provides but don't have names associated with
    // their fields; rather, they just have the types of the fields. Tuple structs are
    // useful when you want to give the whole tuple a name and make the tuple a different
    // type from other tuples, and when naming each field would be verbose or redundant.

    let black = Color(0, 0, 0);
    let home = Point(12, 125, 32);

    // Unit-Like Struct Without Any Field
    // We can also define structs that don't have any fields, These are called unit-like struct
    // because they behave similarly to the unit type. Unit-like structs can be useful when you
    // need to implement as trait on some type but don't have any data that you want to store
    // in the type itself.
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
