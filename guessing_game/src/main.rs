use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number: {secret_number}");

    loop {
        println!("Guess a number");
    
        println!("Input a number:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) //read_line appends stdin onto the specified string
            .expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("Please type a number.");
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => { 
                println!("You win");
                break;
            }
        }
    }
}
