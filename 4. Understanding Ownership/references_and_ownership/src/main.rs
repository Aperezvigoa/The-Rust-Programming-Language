// - REFERENCES AND BORROWING
// Ownership, boxes and moves provide a foundation for safely
// programming with the heap. However, move-only can be inconvent
// to use.

fn main() {
    let m1: String = String::from("Hello");
    let m2: String = String::from("Rust!");

    greet(m1, m2);
    // Here, greet1 = m1, so m1 lose ownership. Same with m2.

    println!("{} {}", m1, m2);
}

fn greet(greet1: String, greet2: String) {
    println!("{} {}", greet1, greet2);
}
