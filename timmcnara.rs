use std::fmt;

struct Greeting {
    name: String
}

impl Greeting {
    fn new(name: &str) -> Self {
        Greeting {
            name: ["Mr.", name].join(" ").to_string()
        }
    }
}

impl fmt::Display for Greeting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hello! {}", self.name)
    }
}

fn main() {
    // let greeting = Greeting { name: String::from("Liz") };
    let greeting = Greeting::new("Liz");
    println!("{}", greeting)
}
