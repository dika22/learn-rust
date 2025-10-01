fn main() {
    loops();
    println!("space for result of another function");
    for_example();
}


fn loops() {
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    println!("The result is {}", result);
}

fn for_example() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
}