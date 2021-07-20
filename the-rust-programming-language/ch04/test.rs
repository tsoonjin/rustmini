fn borrow(banner: &str) {
   println!("{}", banner)
}

fn main() {
    // Ownership moved to S2
    let s = String::from("Ben");
    borrow(&s);
    println!("{}", s);
}
