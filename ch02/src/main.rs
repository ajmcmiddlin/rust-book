use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let the_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");

    loop {
        println!("Enter your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess");

        let guess: u32 = guess
            .trim()
            .parse()
            .expect("Unable to parse guess as number");

        match guess.cmp(&the_number) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}
