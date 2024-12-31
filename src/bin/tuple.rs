#![allow(unused)]

// Compund data types
// - tuple
// - array

fn main() {
    // Tuple
    let t: (bool, u32, char) = (true, 1, 'c');

    // Destructure
    let (a, b, c) = t;

    // ignore with _
    let (_, b, _) = t;

    println!("{} {} {}", a, b, c);

    // Access by index
    println!("{} {} {}", t.0, t.1, t.2);

    // Tuple with single element
    let t: (i32,) = (1,);

    // Empty tuple
    let t = ();

    // Nested tuple
    let nested = ((1.23, 'a'), (true, 1u32, 'b'));
    println!("nested {:?}", nested.0 .0);
    println!("nested {:?}", nested.1 .2);
}
