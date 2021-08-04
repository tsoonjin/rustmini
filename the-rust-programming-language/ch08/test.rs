use std::collections::HashMap;

fn main() {
    let mut v = vec!["I", "am", "EGM"];
    println!("{:?}", v);
    let myname = v.get(100);
    match myname {
        Some(s) => println!("My name is {}", s),
        None => println!("Nameless")
    }
    for i in &mut v {
        // let tmp = i.to_owned() + " boo";
        *i = concat!(*i, "boo");
        println!("{}", i);
    }
}
