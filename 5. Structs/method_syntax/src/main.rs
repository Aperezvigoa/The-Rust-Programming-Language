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
    fn can_hold(self: &Self, rect: &Rectangle) -> bool {
        self.area() > rect.area()
    }
    // Associated function
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// We choose &Self because we dont want to take ownership and we just want to read data in the
// struct. If we need to write, we can call &mut Self.
// The main reason for using methods instead of functions, is for organization. We've put all
// the things we can do with an instance of a type in one impl block rather than making future
// users of our code search for capabilities of Rectangle in various places.

// Associated Functions
// All functions defined within an impl block are called associated function because they are
// associated with the type named after the impl. We can define associated functions as functions
// as functions that don't have self as their first parameter because they don't need an instance
// of the type to work with. We've already used one function like this, the String::from function
// that's defined on the String type.
// Associated functions that aren't methods are often used for constructors that will return a
// new instance of the strut. These are often called new, but new is not a special name and isn't
// built into the language.

// Multiple impl Blocks
// Each struct is allowed to have multiple impl blocks:

impl Rectangle {
    // Associated function
    fn standar() -> Self {
        Self {
            width: 40,
            height: 20,
        }
    }
}

// There's no reason to separate these methods into multiple impl blocks here, but is a valid.

// Method Call are Syntactic Sugar for Function Calls
// Using all this concepts, we can now see how method calls are syntactic sugar for function calls
// For example, let's create a set_width method

impl Rectangle {
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let rect1_area = rect1.area();
    println!("The rectangle1 area is: {rect1_area}");

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(3);
    println!("{square:?}");

    let mut rect4 = Rectangle::standar();
    rect4.set_width(180);
    Rectangle::set_width(&mut rect4, 120);
    println!("Rectangle4: {rect4:?}");
}
