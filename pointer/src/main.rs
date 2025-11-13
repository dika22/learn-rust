fn main() {
    let number : i32 = 22;
    println!("number: {}", number);

    let reference = &number;
    println!("reference: {:p}", reference);

    println!("deref: {:}", *reference);
}
