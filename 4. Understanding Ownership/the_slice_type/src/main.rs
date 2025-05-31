// --- The Slice Type
// Slices let you reference a contiguos sequence of elements in a collection rather than the whole collection. A slice is a kind of
// reference, so it is a non-owning pointer.
// To motivate why slices are useful, let's work through a small programming problem, writing a function that takes a string of words
// separated by spaces and return the first word it finds in that string. If the function doesn¡t find a space in the string, the whole
// string must be one word, so the entire string should be returned.
// Example without slices.

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn main() {}
