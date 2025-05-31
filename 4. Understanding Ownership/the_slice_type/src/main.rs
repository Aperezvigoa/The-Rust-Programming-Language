// --- The Slice Type
// Slices let you reference a contiguos sequence of elements in a collection rather than the whole collection. A slice is a kind of
// reference, so it is a non-owning pointer.
// To motivate why slices are useful, let's work through a small programming problem, writing a function that takes a string of words
// separated by spaces and return the first word it finds in that string. If the function doesn¡t find a space in the string, the whole
// string must be one word, so the entire string should be returned.
// Example without slices.

fn _first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// The first_word function returns the index of the space.
// Because we need to go through the String element by element and check whether a value is a space, we'll convert out String to an array
// if bytes using the as_bytes method:
// let bytes = s.as_bytes();
// Next, we create an iterator over the array of bytes using the iter method:
// for (i, &item) in bytes.iter().enumerate() { ... }
// iter is a method that return each elemt in a collection and that enumerate wraps the result of iter and return each element as part of
// a tuple instead. The first element of the tuple returned from enumerate is the index, and the second element is a reference to the
// element.
// Because the enumerate method returns a tuple, we can use patterns to destructure that tuple. In the for loop, we soecify a pattern that
// has i for the index in the tuple and &item for the single byte in the tuple. Because we get a reference to the element from
// .iter().enumerate(), we use & in the pattern.
// Inside the for loop, we search for the byte that represent the space by using the byte literal syntax. If we find a space, we return
// the position. Otherwise, we return the length of the string by using s.len()

fn main() {
    // String Slices
    // A string slice is a reference to part of a String, and it looks like this:

    let s = String::from("Hello world");

    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];

    println!("{} {}", hello, world);
    #[allow(unused_variables)]
    let s2: &String = &s;

    // Rather than a reference to the entire string like s2, hello is a reference to a portion of the String, specified in the extra [0..5]
    // We create slices using a range within braket by specifying [starting_index..ending_index], where starting_index is the first
    // portion in the slice and ending_index is one more than the last position in the slice.
    // Slices are special kinds of references because they are "fat" pointers, or pointers with metadata. Here, the metadata is the length
    // of the slice.

    // Range Syntax
    // With Rust's .. range syntax, if you want to start at index zero, you can drop the value before the two periods. In other words:

    let name: String = String::from("Aperezvigoa");
    let slice_one: &str = &name[0..2];
    let slice_two: &str = &name[..2];

    // By the same token, if your slice includes the last byte of the String, you can drop the trailing number:

    let slice_three: &str = &name[3..len];
    let slice_four: &str = &name[3..];

    // String Literals Are Slices
    // Recall that we talked about string literals being stored inside the binary. Now that we now about slices, we can properly
    // understand string literals.

    let s = "Hello world!";

    // The type of s here is &str: it's a slice pointing to that specific point of the binary. This is also why string literals are
    // immutable; &str is an immutable reference.

    // String Slices as Parameters
    // Knowing that you can take slices of literals and String values leads us to one more improvement on first_word_slices:
    // If we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the String or a reference
    // to the String. This flexibility takes adventage of deref coercions.
}

// Rewriting first_word with string slices
// With all this information in mind, let's rewrite first_word to return a slice- The type that signifies String slice is written as &str

fn first_word_slices(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// We get the index for the end of the word in the same way we did, by looking for the first occurrence of a space. When we find a space,
// we return a string slice using the star of the string and the index of the space as the starting and ending indices.
