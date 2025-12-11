use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = generate_secret_number();

    loop {
        let user_guess = take_user_input();
        let comparison = compare_input_with_secret_number(secret_number, user_guess);
        if comparison {
            break;
        }
    }
}

fn generate_secret_number() -> u32 {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    return secret_number;
}

fn take_user_input() -> u32 {
    loop {
        println!("Please input your guess.");
        let mut user_guess = String::new();

        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read line");

        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Your guess must be a number!\n");
                continue;
            }
        };

        println!("You guessed: {user_guess}");
        return user_guess;
    }
}

fn compare_input_with_secret_number(secret_number: u32, user_guess: u32) -> bool {
    match user_guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small!\n");
            return false;
        }
        Ordering::Greater => {
            println!("Too big!\n");
            return false;
        }
        Ordering::Equal => {
            println!("You win!\n");
            return true;
        }
    }
}
