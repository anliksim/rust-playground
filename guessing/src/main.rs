use rand::Rng;
use std::{cmp::Ordering, io};

const MIN: u32 = 1;
const MAX: u32 = 100;

fn main() {
    println!("Guess the number between {MIN} and {MAX}!");

    let secret_number = rand::thread_rng().gen_range(MIN..=MAX);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim() == "exit" {
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("Bye!");
}
