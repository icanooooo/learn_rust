use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number !");

    let secret_number = rand::rng().random_range(1..=100);
 
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // ini apa?
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        println!("You guessed: {guess}");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("EXACTLY RIGHT!!");
                break;
                } 
        }
    }
    
}
