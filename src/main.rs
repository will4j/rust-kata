use std::io::{self};

use rand::Rng;

fn main() -> io::Result<()> {
    let secret_number = rand::thread_rng().gen_range(1..=1);
    println!("Hello World: {}", secret_number);
    Ok(())
}