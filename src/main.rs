use std::io;
use rand::Rng;

fn main(){

    // generating a random number
    let random_number = rand::thread_rng().gen_range(1..10);
    println!("Welcome to guessing game!");
    println!("Input your guess...");
    
    let mut guess = String::new();
    
    let guess_number: u32 = guess.trim().parse().expect("Not a valid number");
    
    loop {
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Something went wrong...");
    
        if random_number == guess_number {
            println!("Your guess is correct!!");
            break;
        } else {
            println!("Your guess is wrong, guess again!!");
        }
    
        println!("Your guess: {}", guess);
        println!("Random number: {}", random_number);
    }
}

