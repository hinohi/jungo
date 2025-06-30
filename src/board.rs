use crate::zobrist::ZobristTable;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stone {
    Black,
    White,
}

impl Stone {
    pub fn opposite(&self) -> Stone {
        match self {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
        }
    }
}

impl fmt::Display for Stone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stone::Black => write!(f, "○"),
            Stone::White => write!(f, "●"),
        }
    }
}

// Fast board using flat array and u8 representation
#[derive(Debug, Clone)]
pub struct Board {
    size: usize,
    grid: Vec<u8>,            // 0 = empty, 1 = black, 2 = white
    captured: (usize, usize), // (black_captured, white_captured)
    zobrist_table: ZobristTable,
    current_hash: u64,
}

const EMPTY: u8 = 0;
const BLACK: u8 = 1;
const WHITE: u8 = 2;

impl Board {
    pub fn new(size: usize) -> Self {
        Board {
            size,
            grid: vec![EMPTY; size * size],
            captured: (0, 0),
            zobrist_table: ZobristTable::new(size),
            current_hash: 0,
        }
    }

    #[inline(always)]
    fn index(&self, x: usize, y: usize) -> usize {
        y * self.size + x
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline(always)]
    pub fn get(&self, x: usize, y: usize) -> Option<Stone> {
        match self.grid[self.index(x, y)] {
            BLACK => Some(Stone::Black),
            WHITE => Some(Stone::White),
            _ => None,
        }
    }

    #[inline(always)]
    fn get_raw(&self, x: usize, y: usize) -> u8 {
        self.grid[self.index(x, y)]
    }

    #[inline(always)]
    fn stone_to_u8(stone: Stone) -> u8 {
        match stone {
            Stone::Black => BLACK,
            Stone::White => WHITE,
        }
    }

    #[inline(always)]
    fn opposite_u8(stone_u8: u8) -> u8 {
        match stone_u8 {
            BLACK => WHITE,
            WHITE => BLACK,
            _ => EMPTY,
        }
    }

    pub fn is_valid_move(&self, x: usize, y: usize, stone: Stone) -> bool {
        if x >= self.size || y >= self.size || self.get_raw(x, y) != EMPTY {
            return false;
        }

        let stone_u8 = Self::stone_to_u8(stone);
        let opponent_u8 = Self::opposite_u8(stone_u8);

        // Fast path: check if we would capture opponent stones
        let (neighbors, neighbor_count) = self.get_neighbors_array(x, y);

        for &(nx, ny) in &neighbors[..neighbor_count] {
            let neighbor_stone = self.get_raw(nx, ny);

            if neighbor_stone == opponent_u8 {
                // Check if opponent group would be captured after our move
                if self.would_capture_after_move(nx, ny, x, y) {
                    return true; // Capturing move is always valid
                }
            }
        }

        // Check if our stone would have at least one liberty
        for &(nx, ny) in &neighbors[..neighbor_count] {
            if self.get_raw(nx, ny) == EMPTY {
                return true; // Has an empty neighbor
            }
        }

        // Check if we connect to a friendly group that has other liberties
        for &(nx, ny) in &neighbors[..neighbor_count] {
            if self.get_raw(nx, ny) == stone_u8 {
                // Check if the friendly group has liberties other than (x,y)
                if self.group_has_liberty_except(nx, ny, x, y) {
                    return true;
                }
            }
        }

        false // Would be suicide without capture
    }

    // Helper method: check if a group would be captured after blocking one liberty
    fn would_capture_after_move(
        &self,
        group_x: usize,
        group_y: usize,
        block_x: usize,
        block_y: usize,
    ) -> bool {
        let stone_u8 = self.get_raw(group_x, group_y);
        if stone_u8 == EMPTY {
            return false;
        }

        let mut visited = vec![false; self.size * self.size];
        !self.has_liberty_except_recursive(
            group_x,
            group_y,
            stone_u8,
            block_x,
            block_y,
            &mut visited,
        )
    }

    // Helper method: check if a group has at least one liberty excluding a specific position
    fn group_has_liberty_except(
        &self,
        x: usize,
        y: usize,
        except_x: usize,
        except_y: usize,
    ) -> bool {
        let stone_u8 = self.get_raw(x, y);
        if stone_u8 == EMPTY {
            return false;
        }

        let mut visited = vec![false; self.size * self.size];
        self.has_liberty_except_recursive(x, y, stone_u8, except_x, except_y, &mut visited)
    }

    fn has_liberty_except_recursive(
        &self,
        x: usize,
        y: usize,
        stone_u8: u8,
        except_x: usize,
        except_y: usize,
        visited: &mut Vec<bool>,
    ) -> bool {
        let idx = self.index(x, y);
        if visited[idx] {
            return false;
        }
        visited[idx] = true;

        let (neighbors, neighbor_count) = self.get_neighbors_array(x, y);
        for &(nx, ny) in &neighbors[..neighbor_count] {
            let neighbor_stone = self.get_raw(nx, ny);

            if neighbor_stone == EMPTY {
                if (nx, ny) != (except_x, except_y) {
                    return true; // Found a liberty
                }
            } else if neighbor_stone == stone_u8
                && self.has_liberty_except_recursive(nx, ny, stone_u8, except_x, except_y, visited)
            {
                return true;
            }
        }

        false
    }

    pub fn place_stone(&mut self, x: usize, y: usize, stone: Stone) -> Result<(), &'static str> {
        if x >= self.size || y >= self.size {
            return Err("Position out of bounds");
        }

        if self.get_raw(x, y) != EMPTY {
            return Err("Position already occupied");
        }

        let stone_u8 = Self::stone_to_u8(stone);
        let idx = self.index(x, y);
        self.grid[idx] = stone_u8;

        // Update Zobrist hash
        self.current_hash ^= self
            .zobrist_table
            .get_stone_hash(x, y, stone == Stone::Black);

        // Check and remove captured stones
        let captured = self.check_captures(x, y, stone);

        // Update capture count
        match stone {
            Stone::Black => self.captured.0 += captured,
            Stone::White => self.captured.1 += captured,
        }

        Ok(())
    }

    fn check_captures(&mut self, x: usize, y: usize, stone: Stone) -> usize {
        let stone_u8 = Self::stone_to_u8(stone);
        let opponent_u8 = Self::opposite_u8(stone_u8);
        let mut total_captured = 0;

        // Check adjacent positions
        let (neighbors, neighbor_count) = self.get_neighbors_array(x, y);

        for &(nx, ny) in &neighbors[..neighbor_count] {
            if self.get_raw(nx, ny) == opponent_u8 {
                let group = self.get_group(nx, ny);
                if self.has_no_liberties(&group) {
                    // Remove the captured group
                    for &(gx, gy) in &group {
                        let idx = self.index(gx, gy);
                        let was_black = self.grid[idx] == BLACK;
                        self.grid[idx] = EMPTY;
                        // Update Zobrist hash for removed stone
                        self.current_hash ^= self.zobrist_table.get_stone_hash(gx, gy, was_black);
                    }
                    total_captured += group.len();
                }
            }
        }

        // Check if the placed stone itself has no liberties (self-capture)
        let self_group = self.get_group(x, y);
        if self.has_no_liberties(&self_group) {
            // Remove the self-captured group
            for &(gx, gy) in &self_group {
                let idx = self.index(gx, gy);
                let was_black = self.grid[idx] == BLACK;
                self.grid[idx] = EMPTY;
                // Update Zobrist hash for removed stone
                self.current_hash ^= self.zobrist_table.get_stone_hash(gx, gy, was_black);
            }
        }

        total_captured
    }

    #[inline(always)]
    fn get_neighbors_array(&self, x: usize, y: usize) -> ([(usize, usize); 4], usize) {
        let mut neighbors = [(0, 0); 4];
        let mut count = 0;

        if x > 0 {
            neighbors[count] = (x - 1, y);
            count += 1;
        }
        if x < self.size - 1 {
            neighbors[count] = (x + 1, y);
            count += 1;
        }
        if y > 0 {
            neighbors[count] = (x, y - 1);
            count += 1;
        }
        if y < self.size - 1 {
            neighbors[count] = (x, y + 1);
            count += 1;
        }

        (neighbors, count)
    }

    fn get_group(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let stone_u8 = self.get_raw(x, y);
        if stone_u8 == EMPTY {
            return vec![];
        }

        let mut group = Vec::new();
        let mut visited = vec![false; self.size * self.size];
        let mut stack = vec![(x, y)];

        while let Some((cx, cy)) = stack.pop() {
            let idx = self.index(cx, cy);
            if visited[idx] {
                continue;
            }

            visited[idx] = true;
            group.push((cx, cy));

            let (neighbors, neighbor_count) = self.get_neighbors_array(cx, cy);
            for &(nx, ny) in &neighbors[..neighbor_count] {
                let nidx = self.index(nx, ny);
                if !visited[nidx] && self.get_raw(nx, ny) == stone_u8 {
                    stack.push((nx, ny));
                }
            }
        }

        group
    }

    fn has_no_liberties(&self, group: &[(usize, usize)]) -> bool {
        for &(x, y) in group {
            let (neighbors, neighbor_count) = self.get_neighbors_array(x, y);
            for &(nx, ny) in &neighbors[..neighbor_count] {
                if self.get_raw(nx, ny) == EMPTY {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_captured(&self) -> (usize, usize) {
        self.captured
    }

    pub fn is_eye(&self, x: usize, y: usize, stone: Stone) -> bool {
        if self.get_raw(x, y) != EMPTY {
            return false; // Already occupied
        }

        let stone_u8 = Self::stone_to_u8(stone);

        // Get all neighbors
        let (neighbors, neighbor_count) = self.get_neighbors_array(x, y);

        // All neighbors must be the same color
        for &(nx, ny) in &neighbors[..neighbor_count] {
            let neighbor_stone = self.get_raw(nx, ny);
            if neighbor_stone != stone_u8 {
                return false; // Different color or empty
            }
        }

        // Check diagonal neighbors for corner/edge cases
        let diagonal_positions = self.get_diagonal_neighbors(x, y);
        let diagonal_count = diagonal_positions.len();
        let mut opponent_diagonal_count = 0;

        for &(dx, dy) in &diagonal_positions {
            let diag_stone = self.get_raw(dx, dy);
            if diag_stone != EMPTY && diag_stone != stone_u8 {
                opponent_diagonal_count += 1;
            }
        }

        // Eye rules based on position:
        match diagonal_count {
            1 => {
                // Corner: the single diagonal must be our color
                for &(dx, dy) in &diagonal_positions {
                    if self.get_raw(dx, dy) != stone_u8 {
                        return false;
                    }
                }
                true
            }
            2 => {
                // Edge: no opponent stones on diagonals
                opponent_diagonal_count == 0
            }
            4 => {
                // Center: max 1 opponent stone on diagonals
                opponent_diagonal_count <= 1
            }
            _ => false,
        }
    }

    fn get_diagonal_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut diagonals = Vec::with_capacity(4);

        // Top-left
        if x > 0 && y > 0 {
            diagonals.push((x - 1, y - 1));
        }
        // Top-right
        if x < self.size - 1 && y > 0 {
            diagonals.push((x + 1, y - 1));
        }
        // Bottom-left
        if x > 0 && y < self.size - 1 {
            diagonals.push((x - 1, y + 1));
        }
        // Bottom-right
        if x < self.size - 1 && y < self.size - 1 {
            diagonals.push((x + 1, y + 1));
        }

        diagonals
    }

    pub fn count_eyes_for_color(&self, stone: Stone) -> usize {
        let mut eye_count = 0;

        for y in 0..self.size {
            for x in 0..self.size {
                if self.is_eye(x, y, stone) {
                    eye_count += 1;
                }
            }
        }

        eye_count
    }

    pub fn get_hash(&self) -> u64 {
        self.current_hash
    }

    // Additional methods for compatibility
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.grid.iter().all(|&cell| cell == EMPTY)
    }

    pub fn count_stones(&self) -> (usize, usize) {
        let mut black_count = 0;
        let mut white_count = 0;

        for &cell in &self.grid {
            match cell {
                BLACK => black_count += 1,
                WHITE => white_count += 1,
                _ => {}
            }
        }

        (black_count, white_count)
    }

    // Check if placing a stone would result in a specific board hash
    pub fn would_result_in_hash(&self, x: usize, y: usize, stone: Stone) -> Option<u64> {
        if !self.is_valid_move(x, y, stone) {
            return None;
        }

        let mut test_board = self.clone();
        if test_board.place_stone(x, y, stone).is_ok() {
            Some(test_board.get_hash())
        } else {
            None
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Print column labels
        write!(f, "   ")?;
        for x in 0..self.size {
            write!(f, "{:2}", x)?;
        }
        writeln!(f)?;

        for y in 0..self.size {
            write!(f, "{:2} ", y)?;
            for x in 0..self.size {
                match self.get(x, y) {
                    None => write!(f, " .")?,
                    Some(stone) => write!(f, " {}", stone)?,
                }
            }
            writeln!(f)?;
        }

        let (black_captured, white_captured) = self.captured;
        writeln!(
            f,
            "Captured: Black={}, White={}",
            black_captured, white_captured
        )?;

        Ok(())
    }
}
