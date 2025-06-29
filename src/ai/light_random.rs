use crate::board::{Board, Stone};
use crate::player::Player;

pub struct LightRandomAI {
    name: String,
    valid_moves_cache: Vec<(usize, usize)>,
}

impl LightRandomAI {
    pub fn new() -> Self {
        LightRandomAI {
            name: "Light Random AI".to_string(),
            valid_moves_cache: Vec::with_capacity(361),
        }
    }
}

impl Default for LightRandomAI {
    fn default() -> Self {
        Self::new()
    }
}

impl LightRandomAI {
    // Get a random valid move without checking eyes
    pub fn get_fast_move(&mut self, board: &Board, stone: Stone) -> Option<(usize, usize)> {
        self.valid_moves_cache.clear();

        // Collect empty positions
        for y in 0..board.size() {
            for x in 0..board.size() {
                if board.get(x, y).is_none() {
                    self.valid_moves_cache.push((x, y));
                }
            }
        }

        if self.valid_moves_cache.is_empty() {
            return None;
        }

        // Try random positions until we find a valid one
        let len = self.valid_moves_cache.len();
        let mut attempts = 0;
        let max_attempts = len.min(20); // Limit attempts to avoid too many validity checks

        while attempts < max_attempts {
            let idx = rand::random::<usize>() % len;
            let (x, y) = self.valid_moves_cache[idx];

            if board.is_valid_move(x, y, stone) {
                return Some((x, y));
            }

            // Remove tried position by swapping with last
            self.valid_moves_cache.swap(idx, len - attempts - 1);
            attempts += 1;
        }

        // If random sampling failed, try all remaining positions
        for i in 0..(len - attempts) {
            let (x, y) = self.valid_moves_cache[i];
            if board.is_valid_move(x, y, stone) {
                return Some((x, y));
            }
        }

        None
    }
}

impl Player for LightRandomAI {
    fn name(&self) -> &str {
        &self.name
    }

    fn get_move(&self, board: &Board, stone: Stone) -> Option<(usize, usize)> {
        // For the Player trait, use the regular RandomAI logic
        // This is just for compatibility - we'll use get_fast_move in simulations
        let mut moves = Vec::new();

        for y in 0..board.size() {
            for x in 0..board.size() {
                if board.is_valid_move(x, y, stone) {
                    moves.push((x, y));
                }
            }
        }

        if moves.is_empty() {
            None
        } else {
            let idx = rand::random::<usize>() % moves.len();
            Some(moves[idx])
        }
    }
}
