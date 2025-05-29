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
    let r_abs1 = i32::abs(**r); // explicit dereference (twice) 
    let r_abs2 = r.abs(); // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len(); // implicit dereference
    assert_eq!(s_len1, s_len2);

    // So, i32::abs expects a i32, so Rust will dereference the
    // value, but we can dereference the box using x.abs().
    // This conversion also works the opposite direction. str::len()
    // expects a &str, so we reference our String s.

    // --- Rust Avoids Simultaneus Aliasing and Mutation
    // Pointers are a porwerful and dangerous feature, because they
    // enable aliasing.
    // Aliasing is accessing the same data trhought different variables
    // We must be careful when we use aliasing combined with mutuation.

    // - - - - - - - - - - - - Disaster examples:
    // * By deallocating the aliased data. leaving the other variable
    //   pointing to deallocated memory.
    // * By mutating the aliased data, invalidating runtime properties
    //   expected by the other variable.
    // * By concurrently mutating tha aliased data, causing a data race
    //   with nondeterministic behavior for the other variable.

    // As a running example, we are going to look at programs using the
    // vector data structure, Vec.
    // Unlike arrays which have a fixed length, vectors have a variable
    // length by storing their elements in the heap.

    let mut using_vec: Vec<i32> = vec![1, 2, 3];
    // The macro vec! creates a vectir with the elements between brackets.
    // One important implementation detail is that vec allocates a heap
    // array of a certain capacity.

    using_vec.push(4);
    println!("{}", using_vec[3]);

    // When we use push, the vector has to create a new allocation with
    // larger capacity, copy all elements over and deallocate the original
    // heap array.

    // --- References Change Permissions on Places
    // The core idea behind the borrow checker is that variables have three
    // kinds of permissions on their data:

    // - Read (R) -> Data can be copied to another location.
    // - Write(W) -> Data can be mutated.
    // - Own  (O) -> Data can be moved or dropped.

    // These permissions dont exist at runtime, only within the compiler.
    // They describe how the compiler "thinks" about your program before the
    // program is executed.
    // By default, a variable has read/own permissions (RO) on its data. If a
    // variable is annotated with let mut, then it also has the write
    // permission. The key idea is that references can temporarilly renove
    // these permissions.
    // A mutable reference removes all permissions to the owner, when the
    // borrow ends, the owner gets all permissions again.

    // --- The Borrow Checker Finds Permission Violations
    // Data should not be aliased and mutated. The goal of these permissions is
    // to ensure that data cannot be mutated if it is aliased. Creating a
    // reference to dara, causes that data to be temporarily read-only until the
    // reference is no longer in use.
    // Rust uses thes permissions in its borrow checker. The borrow checker looks
    // for potentially unsafe operations involving references.
    // Unsafe program example:
    /* ================================
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    v.push(4);
    println!("Third element is: {*num}");
    ================================== */
    // This code is unsafe because v loses Write and Own permissions temporarily,
    // We use v.push without write permissions and with num reference in use.
    // And if we use v.push when num is in use, num could be invalidated by push.
    // Rust catches the potential violation of memory safety.

    // --- Mutable References Provide Unique and Non-Owning Access to Data
    // The references we have seen so far are read-only immutable references.
    // Immutable references permit aliasong but disallow mutuation. However, its
    // useful to temporarily provide mutable access to data without moving it.

    let mut another_vector: Vec<i32> = vec![1, 2, 3]; // R + W + O
    let num: &mut i32 = &mut another_vector[2];
    *num += 1;
    println!("Third element is {}", num);
    println!("Vector is now {:?}", another_vector);

    // A mutable reference is created with &mut operator. The type of num is
    // written as &mut i32. Compared to immutable references, we have two
    // important differences in the permission:

    // * When num was immutable reference, the vector still had the READ permission
    //   Now, the vector has lost all permisions while num is in use.
    // * When num was an immutable reference, the place *num only had the READ
    // permission. Now, *num has also gained the WRITE permission.

    // The first observation is what makes mutable references safe. Mutable references
    // allow mutuation but prevent aliasing. The borrowed place vector becomes
    // temporary unusable, so effectively not an alias.

    // The second observation is what makes mutable references useful. vector[2] can be
    // mutated through *num. For example, *num += 1 mutates vector[2]. num cannot be
    // reassigned to a different mutable reference.

    // Mutable references can also temporarily be downgraded to read-only references:

    let mut another_vector: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut another_vector[2];
    let num2: &i32 = &*num;
    println!(
        "Downgraded\nOriginal reference: {}\nnew reference: {}",
        *num, *num2
    );
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
