use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {

        println!("Please enter your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too Small".red()), //Makes it so that the text appears red in the console if answer is wrong.
            Ordering::Greater => println!("{}", "Too Big".red()),
            Ordering::Equal => { 
                println!("{}", "You Win!".green());
                break;
            }
        }
    }
    //println!("The secret number is: {}", secret_number);
}
