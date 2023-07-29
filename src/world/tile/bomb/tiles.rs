// tile.rs

use crate::is_debug;

/// Enum describing a Minesweeper tile
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tiles {
    /// Is a bomb
    Bomb,
    /// Is a bomb neighbor
    BombNeighbor(u8),
    /// Empty tile
    Empty,
}

impl Tiles {
    /// Is the tile a bomb?
    pub const fn is_bomb(&self) -> bool {
        matches!(self, Self::Bomb)
    }

    pub fn console_output(&self) -> Option<String> {
        if is_debug() {
            use colored::Colorize;
            Some(format!(
                "{}",
                match self {
                    Tiles::Bomb => "*".bright_red(),
                    Tiles::BombNeighbor(v) => match v {
                        1 => "1".cyan(),
                        2 => "2".green(),
                        3 => "3".yellow(),
                        _ => v.to_string().red(),
                    },
                    Tiles::Empty => " ".normal(),
                }
            ))
        } else {
            None
        }
    }
}
