// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `zustlings hint primitive_types3` for hints!


fn main() {
    let a = ["Are we there yet?"; 100];
    a;
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
