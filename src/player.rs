use crate::board::{Board, Stone};
use std::io::{self, Write};

pub trait Player {
    fn name(&self) -> &str;
    fn get_move(&self, board: &Board, stone: Stone) -> Option<(usize, usize)>;
}

pub struct HumanPlayer {
    name: String,
}

impl HumanPlayer {
    pub fn new() -> Self {
        HumanPlayer {
            name: "Human".to_string(),
        }
    }
}

impl Default for HumanPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl Player for HumanPlayer {
    fn name(&self) -> &str {
        &self.name
    }

    fn get_move(&self, board: &Board, _stone: Stone) -> Option<(usize, usize)> {
        loop {
            print!("Enter your move (e.g., 'D4' or 'pass'): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            if input == "pass" {
                return None;
            }

            if input.len() < 2 {
                println!("Invalid input. Please enter a coordinate like 'D4' or 'pass'.");
                continue;
            }

            let col_char = input.chars().next().unwrap();
            let row_str = &input[1..];

            if col_char < 'a' || col_char > (b'a' + board.size() as u8 - 1) as char {
                println!(
                    "Invalid column. Please use letters A-{}.",
                    (b'A' + board.size() as u8 - 1) as char
                );
                continue;
            }

            let col = (col_char as u8 - b'a') as usize;

            match row_str.parse::<usize>() {
                Ok(row_num) => {
                    if row_num < 1 || row_num > board.size() {
                        println!("Invalid row. Please use numbers 1-{}.", board.size());
                        continue;
                    }

                    let row = board.size() - row_num;

                    if board.is_valid_move(col, row, _stone) {
                        return Some((col, row));
                    } else {
                        println!("Invalid move! That position is either occupied or would be suicide without capturing.");
                    }
                }
                Err(_) => {
                    println!("Invalid row number.");
                }
            }
        }
    }
}
