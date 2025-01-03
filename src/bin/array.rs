#![allow(unused)]

// compound data types
// - tuple
// - array

fn main() {
    // Array - fixed length, known at complie time
    let arr: [u32; 3] = [1, 2, 3];
    println!("arr[2] = {}", arr[2]);

    let mut arr: [u32; 3] = [1, 2, 3];
    arr[1] = 9;

    let arr: [u32; 10] = [0; 10]; // [0,0,0,0,0,0,0,0,0,0]

    // Slice - length not known at compile time
    let nums: [i32; 10] = [-1, 1, -2, 2, -1, 2, -3, 5, 4, 5];

    // First 3 elements
    let slice = &nums[..3];
    println!("first 3 elements = {:?}", slice);

    // Middle 4 elements
    let s = &nums[3..7];
    println!("middle 4 elements = {:?}", s);

    // Last 3 elements
    let s = &nums[7..];
    println!("last 3 elements = {:?}", s);

    // All elements
    let s = &nums[..];
    println!("all elements = {:?}", s);
}
