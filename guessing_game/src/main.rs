use std::io;
use rand::Rng;

fn main() {
    println!("Guess a number");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret number: {secret_number}");

    println!("Input a number:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) //read_line appends stdin onto the specified string
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
