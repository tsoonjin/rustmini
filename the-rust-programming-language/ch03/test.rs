fn sum(x: i32, y: i32) -> i32 {
    return x + y;
}

fn makeEven(x: i32) -> i32 {
    return if x % 2 == 0 { x } else { x + 1 }
}

fn main() {
    let a: i32 = makeEven(sum(2, 5));
    println!("{}", a);
}
