#![allow(unused)]

// String and &str
fn main() {
    // String = vector of u8 (Vec<u8>) valid UTF-8
    // &str = slice of u8 (&[u8]) valid UTF-8

    // When to use String over &str?
    // String -> mutate or data needs to be owned
    // &str -> read-only

    // String
    let msg: String = String::from("Hello Rust 🦀");
    let len: usize = msg.len();
    println!("msg = {msg}, len = {len}");

    // str - string slice
    // &str
    // - usually used str with reference (borrowed)
    // - immutable

    let msg: String = String::from("Hello Rust 🦀");
    let s: &str = &msg[0..5];
    let len: usize = s.len();
    println!("slice: {s}");
    println!("len: {len}");

    // String literal
    // - stored inside binary
    // - slice pointing to a specific part of the binary
    // - immutable because hard-coded inside binary
    let hello: &str = "Hello Rust";

    let s: &str = r#"
    {
        "name": "John Doe",
        "age": 30
    }
    "#;
    println!("{s}");

    // Deref coercion
    let msg: String = String::from("Hello Rust 🦀");
    let s: &str = &msg;

    // Add &str to String
    let mut msg: String = "Hello Rust".to_string();
    msg += "!";
    println!("{msg}");
    
    // 
    let lang = "Rust";
    let emoji = "🦀";
    let msg = format!("Hello {lang} {emoji}");
    println!("{msg}");

     

    // // String - growable, heap-allocated data structure
    // let mut s = String::new();
    // let data = "initial contents";
    // let s = data.to_string();
    // let s = "initial contents".to_string();
    // let s = String::from("initial contents");

    // // Updating a String
    // let mut s = String::from("foo");
    // s.push_str("bar");
    // println!("{}", s);

    // let mut s = String::from("lo");
    // s.push('l');
    // println!("{}", s);

    // // Concatenation
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;
    // println!("{}", s3);

    // // Concatenation with format!
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // let s = format!("{}-{}-{}", s1, s2, s3);
    // println!("{}", s);

    // // Indexing
    // let s1 = String::from("hello");
    // // let h = s1[0]; // error: cannot be indexed by integer

    // // Slicing
    // let hello = "Здравствуйте";
    // let s = &hello[0..4];
    // println!("{}", s);

    // // Iterating over strings
    // for c in "नमस्ते".chars() {
    //     println!("{}", c);
    // }

    // for b in "नमस्ते".bytes() {
    //     println!("{}", b);
    // }
}
