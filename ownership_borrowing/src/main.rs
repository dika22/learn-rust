fn main() {
    do_something();
}

fn do_something() {
    let mut k = String::from("hello");
    let s = String::from("ini buku");
    let r = &s; // borrow
    {
        let m = String::from("hello world");
        let n = String::from("from rust");
        k = n;
        
        println!("satu: {:?}", m);
    }
    println!("dari ownership: {}, di brorrow: {}", s, r); // ✔️ aman, cuma baca
    println!("dua: {:?}", k);
}