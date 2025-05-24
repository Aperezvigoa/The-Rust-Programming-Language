fn main() {
    let x: bool = true;
    read(x);

    /*
    Unsafe execution example:
    read(x); // x is not defined!
    let x: bool = true;
     */


    // --- Example of variables living in the stack

    let a: i32 = 5;
    let b: i32 = plus_one(a);
    println!("The value of b is {b}");

    // What's happening behind the hood?
    // The first stack frame hold a = 5
    // The second stack frame holds a = 5 and plus_one(5)
    // The third stack frame holds a = 5 and b = 6

    // When an expressions read a variable,
    // the variable's value is copied from its slot in the stack frame.

    let a: i32 = 5;
    let mut b: i32 = a;
    b += 1;
    println!("{b}");

    // First stack frame holds a = 5
    // Second stack frame holds a = 5 and b = 5
    // Third stack frame holds a = 5 and b = 6

    // --- Box Live in the Heap
    // Copying data can take up a lot of memory. For example, let's take an array
    // of 1 million and copy it into another value

    let a = [0; 1_000_000];
    #[allow(unused_variables)]
    let b = a;

    // First stack frame holds [a = 0, 0, 0, 0, 0, 0 , 0 ... 0]
    // Second stack frame hold:
    // [a = 0, 0, 0, 0, 0, 0, 0 ... 0]
    // [b = 0, 0, 0, 0, 0, 0, 0 ... 0]
    // So, in the second stack frame, we have 2 million elements

    // To transfer data without copying it, Rust uses pointers.
    // A pointer is a value that describes a location in memory.
    // The value that a pointer points-to is called its pointee.
    // One common way to make a pointer is to allocate memory in
    // the heap.
    // The heap is a separate region of memory where data can live
    // indefinitely. Heap data is not tied to a specific stack frame
    // Rust provides a construct called Box for putting data on the
    // heap.

    let a = Box::new([0; 1_000_000]);
    let b = a;
    // dbg!("{}", b); -> Dont run it xD
    println!("{}", b[0]);
    
    // a and b are on the stack, but the array is on the heap.
    // The stack holds data associated with a specific function, while
    // the heap holds data that can outlive a function

    // --- A Box’s Owner Manages Deallocation

    // If a variable is bound to a box, when Rust deallocates the
    // variable's frame, the Rust deallocate the box's heap memory.

    let a_num: i32 = 4;
    println!("a_num -> {}", a_num);
    make_and_drop();
    println!("Box deallocated successfully!");

    // Before make_and_drop() is executed, our stack memory only contains a_num
    // When make_and_drop() executes a_box variable is added to a stack frame
    // and 5 is assigned into the heap
    // When make_a_drop() finishes its execution, a_box is deleted and then
    // Rust deallocates the heap data.
    // After this the heap is empty

    // --- Collections Use Boxes

    let first_name = String::from("Alex");
    let full_name = add_suffix(first_name);

    println!("{full_name}");

    // This example is more complex, so let's check it
    // 1 -
    // In the first string "Alex" is allocated on the heap, it is owned by first_name
    // 2 -
    // Then we call add_suffix(first_name), this moves ownership of String from first_name
    // to name. The string data is not copied, but the pointer to the data is copied.
    // 3-
    // The function name.push_str("Jr.") resizes the string's heap allocation. This does
    // three things. First, it creates a new larger allocation. Second, it writes "AlexJr."
    // into the new allocation, and third, it frees the original heap memory. first_name now
    // points to deallocated memory.
    // 4-
    // Then the frame, add_suffix is gone, this function returned name, transferring ownership
    // of the string to full_name
}

fn add_suffix(mut name: String) -> String {
    name.push_str("Jr.");
    name
}

fn make_and_drop() {
    #[allow(unused_variables)]
    let a_box = Box::new(5);
}

fn plus_one(n: i32) -> i32 {
    n + 1
}

fn read(y: bool) {
    if y {
        println!("y is true!");
    }    
}
