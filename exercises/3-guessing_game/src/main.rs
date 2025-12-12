use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);
    // rand. is an external crate that provides random number generation.
    // rng() creates a random number generator.
    // random_range(1..=100) generates a random number between 1 and 100, inclusive.

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // Read the input user || saves it in the mutable guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Conditional. Comprove that type guess is u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guessed: {guess}");

        // Compares the secret_number - guess
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
