// Method Syntax
// Methods are similar to functions: we declare them with the fn keyword and a name, the can
// have parameters and a return valu, and they contain some code that's run when the method
// is called from somewhere else. Methods are defined within the context of a struct (enum
// or a trait object) and their first parameter is always self, which represents the instance
// of the struct the method is being called on.

// Defining Methods
// Lets make a Rectangle struct and add a nethod to it to calculate the area.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// To define a Rectangle method, we need to use the impl (implementation) keyword. Everything
// inside this impl block will be associated with the Rectangle type. The parameter must be
// &self in the signature and everywhere within the body.
// In the signature for area, we use &self, that is actually short for self: &Self. Within an
// impl block, the type Self is an alias for the type that the impl block is for. Methods must
// have a parameter named self of type Self for their FIRST parameter. We still need to use &
// in front of the Self to indicate that this method borrows the Self instance. Methods can
// take ownership of self, borrow self immutably, as we've done here, or borrow self mutably,
// just as they can any other parameter.

impl Rectangle {
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect1_area = rect1.area();
    println!("The rectangle area is: {rect1_area}");
}
