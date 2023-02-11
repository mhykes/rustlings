// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.

// I AM DONE

fn main() {
    let a:[i32; 101] = [
        1,2,3,4,5,6,7,8,9,10,
        1,2,3,4,5,6,7,8,9,10,
        1,2,3,4,5,6,7,8,9,10,
        1,2,3,4,5,6,7,8,9,10,
        1,2,3,4,5,6,7,8,9,10,
        1,2,3,4,5,6,7,8,9,10,
        1,2,3,4,5,6,7,8,9,10,
        1,2,3,4,5,6,7,8,9,10,
        1,2,3,4,5,6,7,8,9,10,
        1,2,3,4,5,6,7,8,9,10,
        101
        ];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
