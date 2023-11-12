use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess(1-100).");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if num > 100 {
                    println!("Please follow the allowable range (1-100)");
                    continue;
                } else {
                    num
                }
            },
            Err(_) => {
                println!("Please provide valid numbers. Try again");
                continue;
            }
        };

        // Code to exit program using input
        if guess == 0 {
            println!("Exit code received. Exiting!");
            break;
        }

        println!("You guessed: {guess}.");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }

    }
    
}
