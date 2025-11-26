pub fn demonstrate_borrowing() {
    println!("\n--- Borrowing Example ---");

    // 1. References and Borrowing
    // - We can create references to a value without taking ownership.
    // - This is called borrowing.

    // Example 1: Immutable Reference
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // &s1 creates a reference, s1 is NOT moved

    println!("The length of '{}' is {}.", s1, len); // s1 is still valid here

    // Example 2: Mutable Reference
    let mut s2 = String::from("hello");
    change(&mut s2); // &mut s2 creates a mutable reference

    println!("Changed string: {}", s2);

    // Rules of References:
    // 1. At any given time, you can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid.
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
