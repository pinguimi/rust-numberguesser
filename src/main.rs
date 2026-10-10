use std::io;

use rand::prelude::*;

fn main() {

    println!("--- Guess the Number! ---");

    let secret_number = rand::rng().random_range(1..=40);

    println! ("{secret_number}");

    loop{
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let guess: i64 = input.trim().parse().unwrap();

        println!("you guessed {guess}");

        if guess > secret_number {
            println!("--- Lower! ---")
        }
        else if guess < secret_number{
            println!("--- Higher! ---");
        }
        else {
            println!("--- correct! : {secret_number}");
            break;
        }
    }
   
}
