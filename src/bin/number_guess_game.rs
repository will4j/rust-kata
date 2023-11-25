use std::cmp::Ordering;
use std::io::{self, Write};

use rand::Rng;

fn main() -> io::Result<()> {
    println!("Let's Play a Number Guessing Game!");

    let (mut min, mut max, secret_number) = new_game();

    loop {
        let prompt = format!("Guess a Number between {} and {}", min, max);
        let guess = input_int(&prompt);

        print!("You guess {guess}, ");
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                min = guess
            }
            Ordering::Greater => {
                println!("Too big!");
                max = guess
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    Ok(())
}

fn new_game() -> (u32, u32, u32) {
    let mut input_str = String::new();
    input(&mut input_str, "New Game");

    let range: Vec<u32> = input_str
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    let min = range[0];
    let max = range[1];
    let secret_number = rand::thread_rng().gen_range(min..=max);

    (min, max, secret_number)
}

fn input_int(prompt: &str) -> u32 {
    let num;
    loop {
        let mut input_str = String::new();
        input(&mut input_str, prompt);

        match input_str.trim().parse() {
            Ok(_num) => {
                num = _num;
                break;
            }
            Err(_) => {
                println!("please input a integer number.");
                continue;
            }
        }
    }
    num
}

fn input(input_str: &mut String, prompt: &str) {
    print!("{prompt}: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(input_str).expect("failed to read line");
}
