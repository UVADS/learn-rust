// try on https://play.rust-lang.org/

use rand::Rng;
use std::io::{self, Write};

fn main() {
    println!("Welcome to the Number Guessing Game!");
    println!("I'm thinking of a number between 1 and 10...");
    
    let secret_number = rand::thread_rng().gen_range(1..=10);
    
    loop {
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        
        if guess < 1 || guess > 10 {
            println!("Please guess between 1 and 10!");
            continue;
        }
        
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too low!"),
            std::cmp::Ordering::Greater => println!("Too high!"),
            std::cmp::Ordering::Equal => {
                println!("You got it! The number was {}", secret_number);
                break;
            }
        }
    }
}
