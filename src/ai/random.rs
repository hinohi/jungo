use crate::board::{Board, Stone};
use crate::player::Player;
use rand::{thread_rng, Rng};

pub struct RandomAI {
    name: String,
}

impl RandomAI {
    pub fn new() -> Self {
        RandomAI {
            name: "Random AI".to_string(),
        }
    }
}

impl Default for RandomAI {
    fn default() -> Self {
        Self::new()
    }
}

impl Player for RandomAI {
    fn name(&self) -> &str {
        &self.name
    }

    fn get_move(&self, board: &Board, stone: Stone) -> Option<(usize, usize)> {
        let size = board.size();
        let mut valid_moves = Vec::with_capacity(size * size);
        let mut non_eye_moves = Vec::with_capacity(size * size);

        // Collect all valid moves and categorize them
        for y in 0..size {
            for x in 0..size {
                if board.is_valid_move(x, y, stone) {
                    valid_moves.push((x, y));

                    if !board.is_eye(x, y, stone) {
                        non_eye_moves.push((x, y));
                    }
                }
            }
        }

        // If there are no valid moves at all, pass
        if valid_moves.is_empty() {
            return None;
        }

        // Count total eyes for our color
        let total_eyes = board.count_eyes_for_color(stone);

        // If we have more than 2 eyes, we can fill some
        let mut rng = thread_rng();
        if total_eyes > 2 && !valid_moves.is_empty() {
            // Prefer non-eye moves, but also consider filling excess eyes
            if !non_eye_moves.is_empty() {
                // 80% chance to play non-eye move, 20% to fill an eye
                if rng.gen_bool(0.8) {
                    let index = rng.gen_range(0..non_eye_moves.len());
                    return Some(non_eye_moves[index]);
                } else {
                    let index = rng.gen_range(0..valid_moves.len());
                    return Some(valid_moves[index]);
                }
            } else {
                // Only eye moves available, and we have more than 2 eyes, so fill one
                let index = rng.gen_range(0..valid_moves.len());
                return Some(valid_moves[index]);
            }
        }

        // If we have 2 or fewer eyes, don't fill them
        if non_eye_moves.is_empty() {
            return None; // Pass to preserve our eyes
        }

        // Play a non-eye move
        let index = rng.gen_range(0..non_eye_moves.len());
        Some(non_eye_moves[index])
    }
}
