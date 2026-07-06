use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret = rand::random_range(1..=100);
    
    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");
    
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number, try again");
                continue;
            }
        };

        println!("You guessed: {guess}");


        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! 🎉");
                break;
            } 
        }   
    }
}