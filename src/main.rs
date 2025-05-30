use std::io;
use rand::{random, Rng};
use colored::*;

fn main(){
    // generating a random number
    let random_number = rand::thread_rng().gen_range(1..10);
    println!("Welcome to guessing game!");
    println!("Input your guess...");
    
    loop {
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Something went wrong...");
    
        // convert string to int
        let guess: u32 = guess.trim().parse().expect("Not a valid number");
        
        let lower: u32 = if guess < random_number {guess + 1} else {1};
        let upper: u32 = if guess > random_number {guess - 1} else {10}; 

        if guess == random_number {
            println!("{}", "Your guess is correct!!".green());
            break;
        } else {
            println!("{}", format!("Wrong guess, Try guessing between {} and {}", lower, upper).yellow());
        }
    
        println!("Your guess: {}", guess);
    }
}

