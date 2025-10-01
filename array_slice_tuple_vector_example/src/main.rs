fn main() {
    arrays();
    println!("spasi for arrays!");
    tuples();
    println!("spasi for tuples!");
    slices_basic();
}


fn arrays() {
    let a = [1, 2, 3, 4, 5];
    println!("this array have {} elements", a.len());
}

fn tuples() {
    let tuple_a = ("jason", 27, ["racing", "working out"], true);
    println!("tuple_a: {:?}", tuple_a);
    println!("tuple_a.0: {}", tuple_a.0);

    for ar in tuple_a.2.iter() {
        println!("value from tuple_a.2: {}", ar);
    }

    if tuple_a.3 {
        println!("tuple_a.3 is true");
    }
}

fn slices_basic() {
    let numbers = [12, 16, 8, 3];
    // variabel numbers isinya array [12, 16, 8, 3]

    let slice_a = &numbers[0..3];
    // meminjam data milik numbers elemen ke-0 hingga sebelum 3 (yaitu 2)
    // hasilnya adalah [12, 16, 8]

    let slice_b = &slice_a[1..=2];
    // meminjam data milik slice_a elemen ke-1 hingga 2
    // hasilnya adalah [16, 8]

    println!("numbers: {:?}", numbers);
    println!("slice_a: {:?}", slice_a);
    println!("slice_b: {:?}", slice_b);
}