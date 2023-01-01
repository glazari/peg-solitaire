//! Move
//!
//! Move represents a to and a from. Its mostly just a convenience
//! to return a list of possible Moves.
//!
//! the file is not called move because it conflicts with the keyword.

use crate::position::Position;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move {
    pub from: Position,
    pub to: Position,
}
