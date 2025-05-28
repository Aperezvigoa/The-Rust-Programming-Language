// - REFERENCES AND BORROWING
// Ownership, boxes and moves provide a foundation for safely
// programming with the heap. However, move-only can be inconvent
// to use.

fn main() {
    let m1: String = String::from("Hello");
    let m2: String = String::from("Rust!");

    // greet(m1, m2);
    // Here, greet1 = m1, so m1 lose ownership. Same with m2.

    // println!("{} {}", m1, m2); Invalid!

    let (m1_again, m2_again) = alternative_greet(m1, m2);
    println!("{} {}", m1_again, m2_again);
    // We can use m1_again and m2_again because they have the
    // ownership now.
    // alternative_greet moves data from m1 and m2, to the
    // function parameters. At the end of the function, it return
    // both values to another variables.

    // --- References are non-owning pointers
    // A reference is a kind of pointer:

    let name: String = String::from("John");
    let lastname: String = String::from("Doe");

    // for referencing a variable we use '&' symbol.
    tell_full_name(&name, &lastname);

    // The ampersand operator creates a reference (or borrow) the var.
    // &name and &lastname, are two variables allocated on the stack,
    // and both points to name and lastname.
    // When tell_full_name is ends, no heap data has been deallocated.
    // Only the stack frame for both references dissapear.
    // Thath means that references are NON-OWNING POINTERS

    println!("name: {name}\nlastname: {lastname}");
    // We can still use the original variables, because they've never
    // been deallocated.

    // --- Dereferencing a Pointer Accesses Its Data
    // The println!() macro works with both types String and &String
    // because both of them implementes the display trait.
    // The underlying mechanism is the dereference operator, written
    // with an asterisk (*)

    let mut x: Box<i32> = Box::new(2); // 2 allocated on the heap.
    let a: i32 = *x;
    // a is just a copy of 2, a = 2 is allocated on the stack.
    // a and x are separated values.
    println!("x: {x}, a: {a}"); // x: 2, a: 2
    *x += 1;
    println!("x: {x}, a: {a}"); // x: 3, a: 2

    let r1: &Box<i32> = &x; //r1 is just a reference to x
    // So if we want to dereference r1 and get the heap value we need
    // to dereference twice.
    #[allow(unused_variables)]
    let b: i32 = **r1;
    // And the same as a, b is just a copy of **r1 = *x = 3

    let r2: &i32 = &*x;
    // r2 is a reference pointing to the heap value directly
    // So only one dereference is needed to read it.
    #[allow(unused_variables)]
    let c: i32 = *r2;

    // Let's see more examples

    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs(); // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r);
    let r_abs2 = r.abs();
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s);
    let s_len2 = s.len();
    assert_eq!(s_len1, s_len2);
}

fn tell_full_name(name: &String, lastname: &String) {
    println!("My name is {} {}", name, lastname);
}

fn _greet(greet1: String, greet2: String) {
    println!("{} {}", greet1, greet2);
}

fn alternative_greet(greet1: String, greet2: String) -> (String, String) {
    println!("{} {}", greet1, greet2);
    (greet1, greet2)
    //Same function but we return 2 String.
}
