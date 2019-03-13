use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn guess() {
    println!("Guess the number! (1 to 100)");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess");
        let mut guess = String::new();
        let err = io::stdin().read_line(&mut guess);
        err.expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
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

