use crate::board::{Board, Stone};
use crate::player::Player;

pub struct Game {
    pub board: Board,
    pub current_turn: Stone,
    pub consecutive_passes: usize,
}

impl Game {
    pub fn new(board_size: usize) -> Self {
        Game {
            board: Board::new(board_size),
            current_turn: Stone::Black,
            consecutive_passes: 0,
        }
    }

    pub fn play(&mut self, player1: &dyn Player, player2: &dyn Player) {
        println!("Game Start!");
        println!("Black: {}", player1.name());
        println!("White: {}", player2.name());
        println!();

        loop {
            println!("{}", self.board);

            let current_player: &dyn Player = match self.current_turn {
                Stone::Black => player1,
                Stone::White => player2,
            };

            println!("{}'s turn ({})", current_player.name(), self.current_turn);

            match current_player.get_move(&self.board, self.current_turn) {
                Some((x, y)) => match self.board.place_stone(x, y, self.current_turn) {
                    Ok(_) => {
                        self.consecutive_passes = 0;
                        println!(
                            "{} plays at {}{}",
                            current_player.name(),
                            (b'A' + x as u8) as char,
                            self.board.size() - y
                        );
                    }
                    Err(e) => {
                        println!("Invalid move: {}", e);
                        continue;
                    }
                },
                None => {
                    println!("{} passes", current_player.name());
                    self.consecutive_passes += 1;

                    if self.consecutive_passes >= 2 {
                        break;
                    }
                }
            }

            self.current_turn = self.current_turn.opposite();
            println!();
        }

        self.end_game();
    }

    fn end_game(&self) {
        println!("\n=== Game Over ===");
        println!("{}", self.board);

        let (black_stones, white_stones) = self.board.count_stones();
        let (black_captured, white_captured) = self.board.get_captured();

        let black_score = black_stones + black_captured;
        let white_score = white_stones + white_captured;

        println!("Final Score:");
        println!(
            "Black: {} stones + {} captured = {}",
            black_stones, black_captured, black_score
        );
        println!(
            "White: {} stones + {} captured = {}",
            white_stones, white_captured, white_score
        );

        if black_score > white_score {
            println!("\nBlack wins by {} points!", black_score - white_score);
        } else if white_score > black_score {
            println!("\nWhite wins by {} points!", white_score - black_score);
        } else {
            println!("\nThe game is a draw!");
        }
    }
}
