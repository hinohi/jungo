use crate::board::{Board, Stone};
use crate::player::Player;

pub struct Game {
    pub board: Board,
    pub current_turn: Stone,
    pub consecutive_passes: usize,
    pub previous_board: Option<Board>,
    pub board_history: Vec<u64>, // Store hashes of all previous board states
}

impl Game {
    pub fn new(board_size: usize) -> Self {
        let board = Board::new(board_size);
        let initial_hash = board.get_hash();
        Game {
            board,
            current_turn: Stone::Black,
            consecutive_passes: 0,
            previous_board: None,
            board_history: vec![initial_hash],
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
                Some((x, y)) => {
                    // First check if the move is valid
                    if !self.board.is_valid_move(x, y, self.current_turn) {
                        println!("Invalid move: Position not valid");
                        continue;
                    }

                    // Clone board to test the move
                    let mut test_board = self.board.clone();
                    if test_board.place_stone(x, y, self.current_turn).is_ok() {
                        let new_hash = test_board.get_hash();

                        // Check Ko rule: see if this board state occurred 2 moves ago
                        // (1 move ago would be opponent's move)
                        let history_len = self.board_history.len();
                        if history_len >= 2 && self.board_history[history_len - 2] == new_hash {
                            println!("Invalid move: Ko rule violation!");
                            continue;
                        }

                        // Move is valid, apply it
                        let board_before_move = self.board.clone();
                        match self.board.place_stone(x, y, self.current_turn) {
                            Ok(_) => {
                                self.consecutive_passes = 0;
                                self.previous_board = Some(board_before_move);
                                self.board_history.push(self.board.get_hash());
                                println!(
                                    "{} plays at {}{}",
                                    current_player.name(),
                                    (b'A' + x as u8) as char,
                                    y + 1
                                );
                            }
                            Err(e) => {
                                println!("Invalid move: {}", e);
                                continue;
                            }
                        }
                    } else {
                        println!("Invalid move: Cannot place stone");
                        continue;
                    }
                }
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
