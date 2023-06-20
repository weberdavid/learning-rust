use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    let secret = rand::thread_rng().gen_range(1..=20);
    
    loop {
        let mut guess = String::new();
        println!("Enter your guess!");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your line!");

        println!("Your guess is: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("You guessed too small!"),
            Ordering::Greater => println!("You guessed to high!"),
            Ordering::Equal => {
                println!("CORRECT!!!!");
                break
            },
        }
    }
}
