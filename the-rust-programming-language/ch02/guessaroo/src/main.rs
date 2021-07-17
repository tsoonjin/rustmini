use std::io;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let min = 1;
    let max = 10;
    let mut attempts = 3;
    let mut guess = String::new();
    let mut guessNumber: i32 = 0;
    let answer = rng.gen_range(min..max+1);

    while attempts > 0 {
        let prompt = format!("Guess a number between {} and {}.\nYou have {} attempt(s) left", min, max, attempts);
        println!("{}", prompt);
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        guessNumber = guess.split_whitespace().collect::<Vec<_>>()[0].parse().unwrap();
        if (guessNumber == answer) {
            println!("Congrats !! You have got the right answer");
            break;
        }
        guess.clear();
        attempts -= 1;
    }
    println!("Better luck next time. The lucky number is: {}", answer)
}
