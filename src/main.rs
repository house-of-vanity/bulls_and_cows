extern crate rand;

use itertools::Itertools;
use rand::Rng;
use std::env;
use std::io::{stdin, stdout, Write};
mod cow;

struct Game {
    _base: u8,
    _board: Vec<u8>,
}

impl Game {
    // Create a new instance
    fn new(base: u8) -> Game {
        Game {
            _base: base,
            _board: vec![],
        }
    }

    // Return the base
    fn base(&self) -> &u8 {
        &self._base
    }

    // Return the board
    fn board(&self) -> &Vec<u8> {
        &self._board
    }

    // Generate a new board
    fn start(&mut self) {
        self._board.clear();
        let mut rng = rand::thread_rng();
        for _i in 0..self._base {
            self._board.push(rng.gen_range(0, 10));
        }
    }

    // Check answer
    fn check(&self, answer: &Vec<u8>) -> Result<(u8, u8), (u8, u8)> {
        if answer.len() != self._base as usize {
            panic!("Wrong parameters count.");
        }
        let uniq_answer: Vec<_> = answer.clone().into_iter().unique().collect();
        let mut cows = 0;
        let mut bulls = 0;
        for i in 0..uniq_answer.len() {
            for p in &self._board {
                if uniq_answer[i] == *p {
                    cows += 1;
                }
            }
        }
        for i in 0..answer.len() {
            if answer[i] == self._board[i] {
                bulls += 1
            }
        }
        if bulls == self._base {
            Ok((cows, bulls))
        } else {
            Err((cows, bulls))
        }
    }
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

fn main() {
    // Parse args
    let args: Vec<String> = env::args().collect();
    let mut base = 4;
    cow::print_cow();
    if args.len() == 2 {
        base = args[1].parse::<u8>().unwrap();
    }
    println!("=> Starting a new game!\n=> Type <help> for help.\n=> Type <answer> for answer.\n=> You can change initial base by passing argument like ./game 5");
    let help_text = format!(
        "+++++++++++++++++
    Computer generate {} digits. You have to guess them.
    Type digits, eg: 1234 and
    computer returns you a number of
    Cows - count of right digits.
    Bulls - count of right and in place digits.
    You have to guess all digits in its places.
+++++++++++++++++",
        base
    );
    let mut game = Game::new(base);
    let mut attempt = 0;
    game.start();
    let mut input = String::new();
    let mut answer: Vec<u8> = vec![];
    loop {
        input.clear();
        answer.clear();
        print!("[{:>03}] Please enter your variant: ", attempt);
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a correct string");
        if let Some('\n') = input.chars().next_back() {
            input.pop();
        }
        if let Some('\r') = input.chars().next_back() {
            input.pop();
        }
        if input.as_str() == "answer" {
            println!("Base: {}, Generated board: {:?}", game.base(), game.board());
            continue;
        } else if input.as_str() == "help" {
            println!("{}", help_text);
            continue;
        }
        remove_whitespace(&mut input);
        match input.parse::<i32>() {
            Err(_) => {
                println!("Invalid input.");
                continue;
            }
            _ => {}
        }
        if input.len() != *game.base() as usize {
            println!("Invalid input, base is {}. Try again.", *game.base());
            continue;
        }
        for letter in input.chars() {
            answer.push(letter.to_digit(10).unwrap() as u8);
        }

        println!("You typed: {:?}", answer);
        attempt += 1;
        match game.check(&answer) {
            Ok(_result) => {
                println!("You won in {} attempts!", attempt);
                attempt = 0;
                &game.start();
            }
            Err(result) => println!("So close! Cows: {}, Bulls: {}", result.0, result.1),
        }
    }
}
