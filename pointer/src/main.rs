fn main() {
    let number : i32 = 22;
    println!("number: {}", number);

    let reference = &number;
    println!("reference: {:p}", reference);

    println!("deref: {:}", *reference);

    // change value with add mutable
    let mut number = 22;
    println!("number: {}", number);
    let reference = &mut number;
    println!("reference: {:p}", reference);
    *reference = 44;
    println!("number: {}", number);
}
