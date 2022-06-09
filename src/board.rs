use druid::Data;
use std::fmt;

/// The board the game is played on
#[derive(Copy, Clone, Data)]
pub struct Board {
    /// The player who is currently moving
    pub turn: Player,
    /// State of the game
    pub rows: [[Option<Player>; 3]; 3],
    /// Result, if any
    pub result: Option<GameResult>,
}

/// Move made ny a player
#[derive(Copy, Clone)]
pub struct Move {
    /// The row the player puts their piece on
    pub row: usize,
    /// The col the player puts their piece on
    pub col: usize,
    /// True if it is the players "final" move, else false
    pub none: bool,
}

/// The result of the game
#[derive(Copy, Clone, Data)]
pub enum GameResult {
    /// If a player won, then which player
    P(Player),
    /// A tie
    Tie,
}

/// The players playing the game
#[derive(Copy, Clone, PartialEq, Data)]
pub enum Player {
    /// The player X
    X,
    /// The player O
    O,
}

impl Board {
    /// Create a new board
    pub fn new() -> Board {
        return Board {
            turn: Player::X,
            rows: [[None; 3]; 3],
            result: None,
        };
    }

    /// Put a piece on the board
    pub fn turn(&mut self, m: Move) -> Result<(), &str> {
        if self.result.is_none() {
            if m.col > 2 || m.row > 2 {
                return Err("Index out of bounds");
            }
            if self.rows[m.row][m.col].is_some() {
                return Err("Already a piece here");
            }
            self.rows[m.row][m.col] = Some(self.turn);
            self.turn = match self.turn {
                Player::X => Player::O,
                Player::O => Player::X,
            };
            self.result = self.check_winner();
        }
        Ok(())
    }

    /// Check if there is a winner
    pub fn check_winner(&self) -> Option<GameResult> {
        let mut winner = None;

        for row in self.rows {
            if equals_three(row[0], row[1], row[2]) {
                winner = Some(GameResult::P(row[0].unwrap()));
            }
        }

        for i in 0..=2 {
            if equals_three(self.rows[0][i], self.rows[1][i], self.rows[2][i]) {
                winner = Some(GameResult::P(self.rows[0][i].unwrap()));
            }
        }

        if equals_three(self.rows[0][0], self.rows[1][1], self.rows[2][2]) {
            winner = Some(GameResult::P(self.rows[1][1].unwrap()));
        }

        if equals_three(self.rows[0][2], self.rows[1][1], self.rows[2][0]) {
            winner = Some(GameResult::P(self.rows[1][1].unwrap()));
        }

        let mut open = 0;
        for row in self.rows {
            for cell in row {
                if cell.is_none() {
                    open += 1;
                }
            }
        }

        if winner.is_none() && open == 0 {
            Some(GameResult::Tie)
        } else {
            winner
        }
    }
}

/// Check if three values equals each other
fn equals_three(a: Option<Player>, b: Option<Player>, c: Option<Player>) -> bool {
    return a == b && a == c && b == c && a.is_some();
}

/// Convert a cell
pub fn cell_to_string(cell: Option<Player>) -> String {
    match cell {
        Some(player) => format!("{}", player),
        None => " ".to_string(),
    }
}

impl GameResult {
    /// Returns the current "score" based on who is winning
    pub fn score(&self, p: Player) -> i32 {
        match self {
            GameResult::P(player) => {
                if p == *player {
                    10
                } else {
                    -10
                }
            }
            GameResult::Tie => 0,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameResult::P(player) => write!(f, "{}", player),
            GameResult::Tie => write!(f, "Tie"),
        }
    }
}
