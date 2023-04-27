use std::cmp::Ordering;
use std::{io, process};
use colored::Colorize;

use rand::Rng;

fn main() {
    let number: u8 = rand::thread_rng().gen_range(0..100);

    println!("Guess a number between 1 and 100! ({})", number);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read input");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match number.cmp(&guess) {
            Ordering::Less => println!("{}", "Lower".red()),
            Ordering::Greater => println!("{}", "Higher".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                process::exit(0);
            }
        }
    }
}
