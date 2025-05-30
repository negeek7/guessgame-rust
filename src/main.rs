use std::io;
use rand::Rng;
use colored::*;
use std::cmp::Ordering;

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

        match guess.cmp(&random_number) {
            Ordering::Less => println!("{}", format!("Your guess is low, try guessing between {} and {}", lower, upper).yellow()),
            Ordering::Greater => println!("{}", format!("Your guess is high, try guessing between {} and {}", lower, upper).yellow()),
            Ordering::Equal => {
                println!("{}", "Your guess is correct!!".green());
                break;
            },

        }

        // if guess == random_number {
        //     println!("{}", "Your guess is correct!!".green());
        //     break;
        // } else {
        //     println!("{}", format!("Wrong guess, Try guessing between {} and {}", lower, upper).yellow());
        // }
    
        println!("Your guess: {}", guess);
    }
}

