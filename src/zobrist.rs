use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

#[derive(Debug, Clone)]
pub struct ZobristTable {
    black_table: Vec<Vec<u64>>,
    white_table: Vec<Vec<u64>>,
}

impl ZobristTable {
    pub fn new(board_size: usize) -> Self {
        let mut rng = StdRng::seed_from_u64(42); // Fixed seed for consistency

        let mut black_table = vec![vec![0u64; board_size]; board_size];
        let mut white_table = vec![vec![0u64; board_size]; board_size];

        for y in 0..board_size {
            for x in 0..board_size {
                black_table[y][x] = rng.gen();
                white_table[y][x] = rng.gen();
            }
        }

        ZobristTable {
            black_table,
            white_table,
        }
    }

    pub fn get_stone_hash(&self, x: usize, y: usize, is_black: bool) -> u64 {
        if is_black {
            self.black_table[y][x]
        } else {
            self.white_table[y][x]
        }
    }
}
