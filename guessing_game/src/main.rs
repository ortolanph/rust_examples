extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();
        println!("Please input your guess...");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");
    
        println!("You guessed {}", guess);
    
        let my_guess: u32 = guess.trim().parse().expect("Type a number!");
    
        match my_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("YOU WIN!");
                break;
            }
        }
    }
}
