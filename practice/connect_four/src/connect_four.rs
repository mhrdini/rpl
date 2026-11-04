const COLS: usize = 7;
const ROWS: usize = 6;
const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (1, 1), (-1, 1)]; // (row, col)

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player {
    Red,
    Yellow,
}

impl Player {
    pub fn other(self) -> Self {
        match self {
            Self::Red => Self::Yellow,
            Self::Yellow => Self::Red,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    Piece(Player),
    Empty,
}

// always implement From rather than Into,
// as Into is derived from it, but not the other way around
impl From<Player> for Cell {
    fn from(player: Player) -> Cell {
        Cell::Piece(player)
    }
}

#[derive(Debug)]
pub enum MoveError {
    OutOfRange,
    ColumnFull,
    GameOver,
}

#[derive(Clone, Copy, Debug)]
pub struct Board {
    cells: [[Cell; COLS]; ROWS],
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: [[Cell::Empty; COLS]; ROWS],
        }
    }

    pub fn drop_piece(&mut self, col: usize, player: Player) -> Result<usize, MoveError> {
        if col >= COLS {
            return Err(MoveError::OutOfRange);
        }

        let row = (0..ROWS)
            .find(|&r| self.cells[r][col] == Cell::Empty)
            .ok_or(MoveError::ColumnFull)?;

        self.cells[row][col] = player.into(); // mutate the copy only
        Ok(row)
    }

    pub fn check_win(&self, row: usize, col: usize) -> bool {
        let piece = self.cells[row][col];
        if piece == Cell::Empty {
            return false;
        }

        // for each direction axis, tally up the count
        for (dr, dc) in DIRECTIONS {
            let total = 1
                + self.count_direction(row, col, dr, dc, piece) // towards positive direction
                + self.count_direction(row, col, -dr, -dc, piece); // towards the reverse direction
            if total >= 4 {
                return true;
            }
        }
        false
    }

    // counts consecutive "piece" cells starting one step away from (row, col)
    fn count_direction(&self, row: usize, col: usize, dr: isize, dc: isize, piece: Cell) -> usize {
        let mut count = 0;
        let mut r = row as isize + dr;
        let mut c = col as isize + dc;

        // continue to increment count by 1
        // while r and c are within board, and
        // Cell at r and c is of the same Cell variant as piece
        while r >= 0
            && r < ROWS as isize
            && c >= 0
            && c < COLS as isize
            && self.cells[r as usize][c as usize] == piece
        {
            // advance further in the direction of dr and dc at each step
            count += 1;
            r += dr;
            c += dc;
        }
        count
    }
    pub fn is_full(&self) -> bool {
        for row in &self.cells {
            for cell in row {
                if *cell == Cell::Empty {
                    return false;
                }
            }
        }
        true
    }
    pub fn get(&self, row: usize, col: usize) -> Cell {
        self.cells[row][col]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    InProgress,
    Won(Player),
    Draw,
}

pub struct Game {
    board: Board,
    current: Player,
    status: Status,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            current: Player::Red,
            status: Status::InProgress,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }
    pub fn current_player(&self) -> Player {
        self.current
    }
    pub fn status(&self) -> Status {
        self.status
    }

    pub fn play(&mut self, col: usize) -> Result<Status, MoveError> {
        if self.status != Status::InProgress {
            return Err(MoveError::GameOver);
        }

        let row = self.board.drop_piece(col, self.current)?;

        self.status = if self.board.check_win(row, col) {
            Status::Won(self.current)
        } else if self.board.is_full() {
            Status::Draw
        } else {
            self.current = self.current.other();
            Status::InProgress
        };

        Ok(self.status)
    }
}
