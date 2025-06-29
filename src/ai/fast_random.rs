use crate::board::{Board, Stone};
use crate::player::Player;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct FastRandomAI {
    name: String,
}

impl FastRandomAI {
    pub fn new() -> Self {
        FastRandomAI {
            name: "Fast Random AI".to_string(),
        }
    }
}

impl Default for FastRandomAI {
    fn default() -> Self {
        Self::new()
    }
}

impl Player for FastRandomAI {
    fn name(&self) -> &str {
        &self.name
    }

    fn get_move(&self, board: &Board, stone: Stone) -> Option<(usize, usize)> {
        let size = board.size();
        let mut empty_positions = Vec::with_capacity(size * size);

        // First, collect all empty positions
        for y in 0..size {
            for x in 0..size {
                if board.get(x, y).is_none() {
                    empty_positions.push((x, y));
                }
            }
        }

        if empty_positions.is_empty() {
            return None;
        }

        // Shuffle the empty positions
        let mut rng = thread_rng();
        empty_positions.shuffle(&mut rng);

        // Try positions in random order until we find a valid move
        // For performance, we'll skip eye checking in fast random
        for &(x, y) in &empty_positions {
            if board.is_valid_move(x, y, stone) {
                return Some((x, y));
            }
        }

        None
    }
}
