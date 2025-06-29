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

#[derive(Debug, Clone)]
pub struct Board {
    size: usize,
    grid: Vec<Vec<Option<Stone>>>,
    captured: (usize, usize), // (black_captured, white_captured)
}

impl Board {
    pub fn new(size: usize) -> Self {
        Board {
            size,
            grid: vec![vec![None; size]; size],
            captured: (0, 0),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Stone> {
        self.grid[y][x]
    }

    pub fn is_valid_move(&self, x: usize, y: usize, stone: Stone) -> bool {
        if x >= self.size || y >= self.size || self.grid[y][x].is_some() {
            return false;
        }

        // Fast path: check if we would capture opponent stones
        let opponent = stone.opposite();
        let (neighbors, neighbor_count) = self.get_neighbors_array(x, y);

        for &(nx, ny) in &neighbors[..neighbor_count] {
            if self.get(nx, ny) == Some(opponent) {
                // Check if opponent group would be captured after our move
                if self.would_capture_after_move(nx, ny, x, y) {
                    return true; // Capturing move is always valid
                }
            }
        }

        // Check if our stone would have at least one liberty
        for &(nx, ny) in &neighbors[..neighbor_count] {
            if self.get(nx, ny).is_none() {
                return true; // Has an empty neighbor
            }
        }

        // Check if we connect to a friendly group that has other liberties
        for &(nx, ny) in &neighbors[..neighbor_count] {
            if self.get(nx, ny) == Some(stone) {
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
        let stone = self.get(group_x, group_y);
        if stone.is_none() {
            return false;
        }

        let mut visited = vec![vec![false; self.size]; self.size];
        !self.has_liberty_except_recursive(
            group_x,
            group_y,
            stone.unwrap(),
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
        let stone = self.get(x, y);
        if stone.is_none() {
            return false;
        }

        let mut visited = vec![vec![false; self.size]; self.size];
        self.has_liberty_except_recursive(x, y, stone.unwrap(), except_x, except_y, &mut visited)
    }

    fn has_liberty_except_recursive(
        &self,
        x: usize,
        y: usize,
        stone: Stone,
        except_x: usize,
        except_y: usize,
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        if visited[y][x] {
            return false;
        }
        visited[y][x] = true;

        let (neighbors, neighbor_count) = self.get_neighbors_array(x, y);
        for &(nx, ny) in &neighbors[..neighbor_count] {
            match self.get(nx, ny) {
                None => {
                    if (nx, ny) != (except_x, except_y) {
                        return true; // Found a liberty
                    }
                }
                Some(s) if s == stone => {
                    if self.has_liberty_except_recursive(nx, ny, stone, except_x, except_y, visited)
                    {
                        return true;
                    }
                }
                _ => {} // Opponent stone
            }
        }

        false
    }

    pub fn is_valid_move_with_ko(
        &self,
        x: usize,
        y: usize,
        stone: Stone,
        previous_board: &Board,
    ) -> bool {
        // First check if the move is valid without Ko
        if !self.is_valid_move(x, y, stone) {
            return false;
        }

        // Create a temporary board to test the move
        let mut test_board = self.clone();
        test_board.place_stone(x, y, stone).unwrap();

        // Check if the resulting board would be identical to the previous board (Ko)
        !self.boards_are_equal(&test_board, previous_board)
    }

    pub fn boards_are_equal(&self, board1: &Board, board2: &Board) -> bool {
        if board1.size != board2.size {
            return false;
        }

        for y in 0..board1.size {
            for x in 0..board1.size {
                if board1.grid[y][x] != board2.grid[y][x] {
                    return false;
                }
            }
        }

        true
    }

    pub fn count_eyes_for_color(&self, stone: Stone) -> usize {
        let mut eye_count = 0;

        for y in 0..self.size {
            for x in 0..self.size {
                if self.grid[y][x].is_none() && self.is_eye(x, y, stone) {
                    eye_count += 1;
                }
            }
        }

        eye_count
    }

    pub fn is_eye(&self, x: usize, y: usize, stone: Stone) -> bool {
        // Check if a position is an eye for the given stone color
        if self.grid[y][x].is_some() {
            return false; // Already occupied
        }

        // Get all neighbors
        let (neighbors, neighbor_count) = self.get_neighbors_array(x, y);

        // All neighbors must be the same color
        for &(nx, ny) in &neighbors[..neighbor_count] {
            match self.get(nx, ny) {
                Some(neighbor_stone) if neighbor_stone != stone => return false,
                None => return false, // Empty neighbor means not an eye
                _ => continue,
            }
        }

        // Check diagonal neighbors for corner/edge cases
        let diagonal_positions = self.get_diagonal_neighbors(x, y);
        let diagonal_count = diagonal_positions.len();
        let mut opponent_diagonal_count = 0;

        for (dx, dy) in diagonal_positions {
            if let Some(diagonal_stone) = self.get(dx, dy) {
                if diagonal_stone != stone {
                    opponent_diagonal_count += 1;
                }
            }
        }

        // Eye rules based on position:
        // - Corner (1 diagonal): need friendly stone on diagonal
        // - Edge (2 diagonals): need all friendly stones on diagonals
        // - Center (4 diagonals): max 1 opponent stone on diagonals
        match diagonal_count {
            1 => opponent_diagonal_count == 0, // Corner
            2 => opponent_diagonal_count == 0, // Edge
            4 => opponent_diagonal_count <= 1, // Center
            _ => false,
        }
    }

    fn get_diagonal_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut diagonals = Vec::new();

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

    pub fn place_stone(&mut self, x: usize, y: usize, stone: Stone) -> Result<(), &'static str> {
        if x >= self.size || y >= self.size || self.grid[y][x].is_some() {
            return Err("Invalid move");
        }

        self.grid[y][x] = Some(stone);

        // Check for captures
        let captured = self.check_captures(x, y, stone);
        match stone {
            Stone::Black => self.captured.0 += captured,
            Stone::White => self.captured.1 += captured,
        }

        Ok(())
    }

    fn check_captures(&mut self, x: usize, y: usize, stone: Stone) -> usize {
        let opponent = stone.opposite();
        let mut total_captured = 0;

        // Check adjacent positions
        let (neighbors, neighbor_count) = self.get_neighbors_array(x, y);

        for &(nx, ny) in &neighbors[..neighbor_count] {
            if self.get(nx, ny) == Some(opponent) {
                let group = self.get_group(nx, ny);
                if self.has_no_liberties(&group) {
                    // Remove the captured group
                    for &(gx, gy) in &group {
                        self.grid[gy][gx] = None;
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
                self.grid[gy][gx] = None;
            }
        }

        total_captured
    }

    // More efficient version that fills a provided array
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
        let stone = match self.get(x, y) {
            Some(s) => s,
            None => return vec![],
        };

        let mut group = Vec::new();
        let mut visited = vec![vec![false; self.size]; self.size];
        let mut stack = vec![(x, y)];

        while let Some((cx, cy)) = stack.pop() {
            if visited[cy][cx] {
                continue;
            }

            visited[cy][cx] = true;
            group.push((cx, cy));

            let (neighbors, neighbor_count) = self.get_neighbors_array(cx, cy);
            for &(nx, ny) in &neighbors[..neighbor_count] {
                if !visited[ny][nx] && self.get(nx, ny) == Some(stone) {
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
                if self.get(nx, ny).is_none() {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_captured(&self) -> (usize, usize) {
        self.captured
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        for row in &self.grid {
            for cell in row {
                if cell.is_some() {
                    return false;
                }
            }
        }
        true
    }

    pub fn count_stones(&self) -> (usize, usize) {
        let mut black = 0;
        let mut white = 0;

        for row in &self.grid {
            for cell in row {
                match cell {
                    Some(Stone::Black) => black += 1,
                    Some(Stone::White) => white += 1,
                    None => {}
                }
            }
        }

        (black, white)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Column labels
        write!(f, "   ")?;
        for i in 0..self.size {
            write!(f, "{} ", (b'A' + i as u8) as char)?;
        }
        writeln!(f)?;

        // Board rows
        for y in 0..self.size {
            write!(f, "{:2} ", self.size - y)?;
            for x in 0..self.size {
                match self.get(x, y) {
                    Some(stone) => write!(f, "{} ", stone)?,
                    None => write!(f, "· ")?,
                }
            }
            writeln!(f, "{:2}", self.size - y)?;
        }

        // Column labels again
        write!(f, "   ")?;
        for i in 0..self.size {
            write!(f, "{} ", (b'A' + i as u8) as char)?;
        }
        writeln!(f)?;

        // Captured stones
        let (black_captured, white_captured) = self.captured;
        writeln!(
            f,
            "\nCaptured: Black {} - White {}",
            black_captured, white_captured
        )?;

        Ok(())
    }
}
