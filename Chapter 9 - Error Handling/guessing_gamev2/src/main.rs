// Based on Chapter 2, I have added an error handler if the
// value is outside the values between 1 and 100.
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // let guess = String::from("-34"); // this will panic because Guess does not allow it

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => Guess::new(num).value, // this checks the value if it is out of bounds
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
// fn guess_here(g: &str) -> Guess {
//     let guess: Guess = match g.trim().parse() {
//         Ok(num) => return Guess::new(num),
//         Err(_) => panic!("Not a number."),
//     };
// }

// #[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if (1..101).contains(&value) {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}
