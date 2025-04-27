use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("Guess the number!\nType \"quit\" to quit the game.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut attempts_left: u8 = 5;

    println!("You have {attempts_left} tries. Please input your guess.");

    while attempts_left != 0 {
        let mut guess: String = String::new();
        print!("\n({attempts_left} tries left) Your guess: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess = guess.trim();
        if let Ok(number) = guess.parse::<u8>() {
            if number > 100 || number == 0 {
                println!("Please enter a number from 1 to 100");
                continue;
            }
            attempts_left -= 1;
            match number.cmp(&secret_number) {
                Ordering::Less => {
                    println!("Too small!");
                }
                Ordering::Greater => {
                    println!("Too big!");
                }
                Ordering::Equal => {
                    println!("You win!");
                    return;
                }
            };
        } else {
            if guess.trim() == "quit" {
                println!("Bye!");
                return;
            }
            println!("Please enter a number from 1 to 100");
        }
    }
    println!("\nYou lose.");
    println!("Correct answer: {secret_number}");
}
