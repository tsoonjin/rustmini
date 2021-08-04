// fn largest<T>(list: &T) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

struct People<T> {
   age: T
}

pub trait Summary {
    fn summarize(&self) -> String;
}

struct Story {
    exposition: &'static str,
    climax: &'static str,
    conclusion: &'static str
}

impl Summary for Story {
    fn summarize(&self) -> String {
        let mut text = String::from("In the beginning, ");
        text.push_str(self.exposition);
        text
    }
}

fn main() {
    // let number_list = vec![34, 50, 25, 100, 65];

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);

    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    // let result = largest(&number_list);
    // println!("The largest number is {}", result);
    let story = Story{ exposition: "We live in a village far away", climax: "Big dragon attack our village", conclusion: "Hero save the world" };
    println!("{}", story.summarize())
}
