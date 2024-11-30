use rand::Rng;
use std::{cmp::Ordering, io};

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got: {value}.");
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input a guess.");
        let mut user_guess = String::new();
        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read line");
        let user_guess: i32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let guess = Guess::new(user_guess);
        println!("You guessed: {user_guess}");
        match guess.value().cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
