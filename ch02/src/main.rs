use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let the_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");

    println!("Enter your guess:");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read guess");

    let guess: u32 = guess
        .trim()
        .parse()
        .expect("Unable to parse guess as number");

    println!("You guessed {}. The number was {}.", guess, the_number);

    let response = match guess.cmp(&the_number) {
        Ordering::Less => "Too low",
        Ordering::Equal => "You win!",
        Ordering::Greater => "Too high",
    };
    println!("{}", response);
}
