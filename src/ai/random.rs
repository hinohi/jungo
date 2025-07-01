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
        let mut non_eye_moves = Vec::with_capacity(20); // Usually don't need more

        // Early exit constants
        const MIN_NON_EYE_MOVES: usize = 10;
        const MAX_SCAN_POSITIONS: usize = 50;
        let mut scanned = 0;

        // Scan board with early exit
        'outer: for y in 0..size {
            for x in 0..size {
                if board.is_valid_move(x, y, stone) {
                    valid_moves.push((x, y));

                    if !board.is_eye(x, y, stone) {
                        non_eye_moves.push((x, y));

                        // Early exit if we have enough non-eye moves
                        if non_eye_moves.len() >= MIN_NON_EYE_MOVES {
                            break 'outer;
                        }
                    }
                }

                scanned += 1;
                // Limit total positions scanned in midgame/endgame
                if scanned >= MAX_SCAN_POSITIONS && !non_eye_moves.is_empty() {
                    break 'outer;
                }
            }
        }

        // If there are no valid moves at all, pass
        if valid_moves.is_empty() {
            return None;
        }

        let mut rng = thread_rng();

        // If we have non-eye moves, prefer them
        if !non_eye_moves.is_empty() {
            // Only count eyes if we might need to fill them
            if non_eye_moves.len() < 3 {
                let total_eyes = board.count_eyes_for_color(stone);

                if total_eyes > 2 {
                    // 80% chance to play non-eye move, 20% to fill an eye
                    if rng.gen_bool(0.8) || non_eye_moves.is_empty() {
                        let index = rng.gen_range(0..non_eye_moves.len());
                        return Some(non_eye_moves[index]);
                    } else {
                        let index = rng.gen_range(0..valid_moves.len());
                        return Some(valid_moves[index]);
                    }
                } else {
                    // Don't fill eyes if we have 2 or fewer
                    let index = rng.gen_range(0..non_eye_moves.len());
                    return Some(non_eye_moves[index]);
                }
            } else {
                // We have plenty of non-eye moves, just pick one
                let index = rng.gen_range(0..non_eye_moves.len());
                return Some(non_eye_moves[index]);
            }
        }

        // If we only have eye moves, check if we should fill them
        let total_eyes = board.count_eyes_for_color(stone);
        if total_eyes > 2 {
            let index = rng.gen_range(0..valid_moves.len());
            Some(valid_moves[index])
        } else {
            None // Pass to preserve eyes
        }
    }
}
