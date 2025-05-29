use std::io;
use rand::Rng;
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

        if guess == random_number {
            println!("{}", "Your guess is correct!!".green());
            break;
        } else if  guess > random_number {
            println!("{}", "Your guess is wrong and too high, guess again!!".red());
        } else {
            println!("{}", "Your guess is wrong and too low, guess again!!".red());
        }
    
        println!("Your guess: {}", guess);
    }
}

