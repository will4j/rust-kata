use std::io::{self, Write};

use rand::Rng;

fn main() -> io::Result<()> {
    println!("Guessing a random number from range!");

    print!("New Game: ");
    io::stdout().flush().unwrap();

    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("input error");

    let range: Vec<u32> = input_str
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    let min = range[0];
    let max = range[1];
    let secret_number = rand::thread_rng().gen_range(min..=max);

    loop {
        print!("Guess a Number range {} to {}: ", min, max);
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("something wrong");
                continue;
            },
        };
        println!("{secret_number} {guess}");
    }

    Ok(())
}