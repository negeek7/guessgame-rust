use std::io;
use rand::Rng;

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
    
        let guess_number: u32 = guess.trim().parse().expect("Not a valid number");

        if guess_number == random_number {
            println!("Your guess is correct!!");
            break;
        } else if  guess_number > random_number {
            println!("Your guess is wrong and too high, guess again!!");
        } else {
            println!("Your guess is wrong and too low, guess again!!");
        }
    
        println!("Your guess: {}", guess);
    }
}

