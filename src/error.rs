//! Crate specific Errors
//!
//! This defines the error types that are used in all the Result functions
//! in the crate.

#[derive(Debug, PartialEq)]
pub enum Error<'a> {
    UnalinedMove(&'a str),
    DistanceNot2Move(&'a str),
    SpacesInvolvedNotCorrect(&'a str),
}

pub const INVALID_MOVE_MESSAGE: &str = "Invalid Move. Valid Moves requre the following conditions:

- Piece must start and end either in the same column or the same row.
- Piece must move 2 positions, skipping over an occupied piece.
- End position must be empty and initial position needs a piece.
";
