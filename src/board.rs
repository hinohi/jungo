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
            Stone::Black => write!(f, "●"),
            Stone::White => write!(f, "○"),
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

    pub fn is_valid_move(&self, x: usize, y: usize) -> bool {
        if x >= self.size || y >= self.size || self.grid[y][x].is_some() {
            return false;
        }
        true
    }

    pub fn is_valid_move_for_stone(&self, x: usize, y: usize, stone: Stone) -> bool {
        if !self.is_valid_move(x, y) {
            return false;
        }

        // Create a temporary board to test the move
        let mut test_board = self.clone();
        test_board.grid[y][x] = Some(stone);

        // Check if this move would capture opponent stones
        let opponent = stone.opposite();
        let neighbors = test_board.get_neighbors(x, y);
        let mut would_capture = false;

        for (nx, ny) in &neighbors {
            if test_board.get(*nx, *ny) == Some(opponent) {
                let group = test_board.get_group(*nx, *ny);
                if test_board.has_no_liberties(&group) {
                    would_capture = true;
                    break;
                }
            }
        }

        // If we would capture opponent stones, the move is valid
        if would_capture {
            return true;
        }

        // Otherwise, check if our group would have liberties
        let self_group = test_board.get_group(x, y);
        !test_board.has_no_liberties(&self_group)
    }

    pub fn place_stone(&mut self, x: usize, y: usize, stone: Stone) -> Result<(), &'static str> {
        if !self.is_valid_move(x, y) {
            return Err("Invalid move");
        }

        self.grid[y][x] = Some(stone);

        // Check for captures
        let captured = self.check_captures(x, y, stone);
        match stone {
            Stone::Black => self.captured.1 += captured,
            Stone::White => self.captured.0 += captured,
        }

        Ok(())
    }

    fn check_captures(&mut self, x: usize, y: usize, stone: Stone) -> usize {
        let opponent = stone.opposite();
        let mut total_captured = 0;

        // Check adjacent positions
        let neighbors = self.get_neighbors(x, y);

        for (nx, ny) in neighbors {
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

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();

        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < self.size - 1 {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < self.size - 1 {
            neighbors.push((x, y + 1));
        }

        neighbors
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

            for (nx, ny) in self.get_neighbors(cx, cy) {
                if !visited[ny][nx] && self.get(nx, ny) == Some(stone) {
                    stack.push((nx, ny));
                }
            }
        }

        group
    }

    fn has_no_liberties(&self, group: &[(usize, usize)]) -> bool {
        for &(x, y) in group {
            for (nx, ny) in self.get_neighbors(x, y) {
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
