// Example Program Using Structs
// To understand when we migth want to use structs, let's write a program that calculates
// the area of a rectangle.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("rect1 is {rect1:?}");
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
