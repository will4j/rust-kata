use std::cmp::Ordering;
use std::io::{self, Write};

use rand::Rng;

#[cfg(test)]
mod tests {
    use crate::{do_guess, new_game};

    // start of basic use cases
    #[test]
    fn new_game_ok() {
        let (min, max, secret_number) = new_game("1 100");
        assert_eq!(min, 1);
        assert_eq!(max, 100);
        assert!(secret_number >= 1 && secret_number <= 100);
    }

    #[test]
    fn do_guess_bingo() {
        let guess_result = do_guess(1, 100, 50, "50");
        assert_eq!(guess_result, Ok((0, 50, 50)));
    }
    // end of basic use cases

    // start of exception use cases
    #[test]
    fn do_guess_wrong() {
        let guess_result = do_guess(1, 100, 37, "50");
        assert_eq!(guess_result, Ok((1, 1, 49)));

        let guess_result = do_guess(1, 49, 37, "25");
        assert_eq!(guess_result, Ok((-1, 26, 49)));
    }

    #[test]
    fn do_guess_failed() {
        let guess_result = do_guess(1, 100, 37, "abc");
        assert_eq!(guess_result.unwrap_err(), "Please input a integer number");

        let guess_result = do_guess(25, 50, 37, "51");
        assert_eq!(guess_result.unwrap_err(), "Number should between 25 and 50");
    }
    // end of exception use cases
}

fn main() -> io::Result<()> {
    println!("Let's Play a Number Guessing Game!");

    print!("New Game: ");
    io::stdout().flush().unwrap();

    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).unwrap();

    // 根据用户输入创建新游戏，返回最小值、最大值和随机数字答案
    let (mut min, mut max, secret_number) = new_game(&input_str);

    // 记录猜的次数
    let mut count: u32 = 0;
    loop {
        print!("Guess a Number between {min} and {max}: ");
        io::stdout().flush().unwrap();

        let mut input_str = String::new();
        io::stdin().read_line(&mut input_str).unwrap();

        let guess_result = do_guess(min, max, secret_number, &input_str);
        match guess_result {
            Ok((result, new_min, new_max)) => {
                print!("You guess {}, ", input_str.trim());
                count += 1;

                min = new_min;
                max = new_max;

                if result == 0 {
                    println!("You win with {count} guesses!");
                    break;
                } else if result == -1 {
                    println!("Too small!");
                } else {
                    println!("Too big!");
                }
            }
            Err(err) => {
                println!("{err}");
                continue;
            }
        };
    }
    Ok(())
}

/// 创建新的猜数字游戏，返回游戏范围以及范围内的随机数字。
///
/// # Arguments
///
/// * `game_str` - 游戏创建字符串，空格分割的数字起始和结束范围。
///
/// # Examples
/// ```
/// let (min, max, secret) = new_game("1 100")
/// ```
fn new_game(game_str: &str) -> (u32, u32, u32) {
    let range: Vec<u32> = game_str
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    let min = range[0];
    let max = range[1];
    let secret_number = rand::thread_rng().gen_range(min..=max);

    (min, max, secret_number)
}

/// 进行一轮猜数字游戏，返回猜测结果以及根据结果调整过后的数字范围。
///
/// 功能包含对输入进行校验。
///
/// # Arguments
///
/// * `min` - 数字范围：最小数字
/// * `max` - 数字范围：最大数字
/// * `secret_number` - 猜数字游戏答案
/// * `guess_str` - 本轮游戏输入
fn do_guess(min: u32, max: u32, secret_number: u32, guess_str: &str) -> Result<(i8, u32, u32), String> {
    match guess_str.trim().parse::<u32>() {
        Ok(_num) => {
            if _num < min || _num > max {
                return Err(format!("Number should between {min} and {max}"));
            }
            // 比对结果，调整数字范围
            match _num.cmp(&secret_number) {
                Ordering::Less => Ok((-1, _num + 1, max)),
                Ordering::Greater => Ok((1, min, _num - 1)),
                Ordering::Equal => Ok((0, _num, _num)),
            }
        }
        Err(_) => return Err(String::from("Please input a integer number"))
    }
}
