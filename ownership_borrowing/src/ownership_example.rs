pub fn demonstrate_ownership() {
    println!("--- Ownership Example ---");

    // 1. Ownership Rules
    // - Each value in Rust has a variable that’s called its owner.
    // - There can only be one owner at a time.
    // - When the owner goes out of scope, the value will be dropped.

    // Example 1: Move (String)
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2

    // println!("{}, world!", s1); // This would cause a compile-time error because s1 is invalid
    println!("s2 = {}", s2); // s2 is valid

    // Example 2: Clone (Deep copy)
    let s3 = String::from("world");
    let s4 = s3.clone(); // s3 is cloned to s4

    println!("s3 = {}, s4 = {}", s3, s4); // Both are valid

    // Example 3: Copy (Integers)
    let x = 5;
    let y = x; // x is copied to y because integers implement the Copy trait

    println!("x = {}, y = {}", x, y); // Both are valid
}
