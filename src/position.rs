//! Position on board
//!
//! This type describes a position on the board. For convenience there
//! are also some helper methods to add and subtract postitions.
use std::ops::{Add, Sub};

/// Describes a position on the board.
#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Sub for Position {
    type Output = (i32, i32);

    fn sub(self, other: Position) -> Self::Output {
        (
            self.x as i32 - other.x as i32,
            self.y as i32 - other.y as i32,
        )
    }
}

impl Add<(i32, i32)> for Position {
    type Output = Position;

    fn add(self, other: (i32, i32)) -> Self::Output {
        Position {
            x: (self.x as i32 + other.0) as usize,
            y: (self.y as i32 + other.1) as usize,
        }
    }
}
