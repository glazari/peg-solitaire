//! Board definition
//!
//! This defines the board and its operations. For now this represents
//! a concreat implementation of the board. Maybe in the future we will
//! want to make the board definition a trait so that we can try out
//! different implementations of it and check for the performance
//! implications

/*
 * Board is a 7x7 matrix
 * Some of the fields are not used.
 */
#[derive(PartialEq, Clone)]
pub struct Board {
    board: [[Space; 7]; 7],
}

use crate::error::Error::*;
use crate::error::{Error, INVALID_MOVE_MESSAGE};
use crate::move_struct::Move;
use crate::position::Position;
use std::fmt;
use Space::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Space {
    Empty,
    Occupied,
    NotPartOfBoard,
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Board Start: {} Board End", self.serialize())
    }
}

/*
 * Board Game methods
 *
 */
impl Board {
    pub fn move_piece(&self, from: Position, to: Position) -> Result<Board, Error> {
        let middle = self.valid_move(Move { from, to })?;
        let mut new_board = (*self).clone();
        new_board.set(from, Empty);
        new_board.set(to, Occupied);
        new_board.set(middle, Empty);

        Ok(new_board)
    }

    /// returns the middle position if valid
    pub(crate) fn valid_move(&self, movement: Move) -> Result<Position, Error> {
        let (to, from) = (movement.to, movement.from);
        let diff = to - from;
        if diff.0 != 0 && diff.1 != 0 {
            return Err(UnalinedMove(INVALID_MOVE_MESSAGE, self.clone(), movement));
        }
        if diff.0.abs() + diff.1.abs() != 2 {
            return Err(DistanceNot2Move(
                INVALID_MOVE_MESSAGE,
                self.clone(),
                movement,
            ));
        }

        let middle: Position = from + (diff.0 / 2, diff.1 / 2);
        if self.at(from) != Occupied || self.at(to) != Empty || self.at(middle) != Occupied {
            return Err(SpacesInvolvedNotCorrect(
                INVALID_MOVE_MESSAGE,
                self.clone(),
                movement,
            ));
        }

        Ok(middle)
    }

    pub(crate) fn at(&self, pos: Position) -> Space {
        self.board[pos.y][pos.x]
    }

    pub(crate) fn set(&mut self, pos: Position, val: Space) {
        self.board[pos.y][pos.x] = val;
    }
}

/*
 * Board constructors
 * they are all static methods
 */
impl Board {
    pub fn new() -> Board {
        #[rustfmt::skip]
        let b = Board {
            board: [
                [NotPartOfBoard,  NotPartOfBoard, Occupied, Occupied, Occupied, NotPartOfBoard,  NotPartOfBoard],
                [NotPartOfBoard,  NotPartOfBoard, Occupied, Occupied, Occupied, NotPartOfBoard,  NotPartOfBoard],
                [Occupied,        Occupied,       Occupied, Occupied, Occupied, Occupied,        Occupied],
                [Occupied,        Occupied,       Occupied, Empty,    Occupied, Occupied,        Occupied],
                [Occupied,        Occupied,       Occupied, Occupied, Occupied, Occupied,        Occupied],
                [NotPartOfBoard,  NotPartOfBoard, Occupied, Occupied, Occupied, NotPartOfBoard,  NotPartOfBoard],
                [NotPartOfBoard,  NotPartOfBoard, Occupied, Occupied, Occupied, NotPartOfBoard,  NotPartOfBoard],
            ]
        };

        b
    }

    pub(crate) fn from_array(board: [[Space; 7]; 7]) -> Board {
        Board { board }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sucessfull_move() {
        let full_board = Board::new();
        let (from, to) = (Position { x: 3, y: 1 }, Position { x: 3, y: 3 });
        // move from 3,1 to 3, 3
        let expected = "
  ***  
  *-*  
***-***
*******
*******
  ***  
  ***  
";
        let got = full_board
            .move_piece(from, to)
            .expect("valid move")
            .serialize();
        assert_eq!(expected, got, "\nExpected: {}\nGot: {}", expected, got);
    }

    #[test]
    fn failed_move_unalined() {
        let full_board = Board::new();
        let (from, to) = (Position { x: 3, y: 2 }, Position { x: 4, y: 5 });

        assert_eq!(
            Err(UnalinedMove(
                INVALID_MOVE_MESSAGE,
                full_board.clone(),
                Move { from, to }
            )),
            full_board.move_piece(from, to)
        );
    }

    #[test]
    fn failed_move_distance_more_than_two() {
        let full_board = Board::new();
        let (from, to) = (Position { x: 3, y: 2 }, Position { x: 3, y: 5 });

        assert_eq!(
            Err(DistanceNot2Move(
                INVALID_MOVE_MESSAGE,
                full_board.clone(),
                Move { from, to }
            )),
            full_board.move_piece(from, to)
        );
    }

    #[test]
    fn failed_move_distance_less_than_two() {
        let full_board = Board::new();
        let (from, to) = (Position { x: 3, y: 2 }, Position { x: 3, y: 3 });

        assert_eq!(
            Err(DistanceNot2Move(
                INVALID_MOVE_MESSAGE,
                full_board.clone(),
                Move { from, to }
            )),
            full_board.move_piece(from, to)
        );
    }

    #[test]
    /// Initial versions had x and y flipped and initial tests were
    /// all simetric relative to this flip, so this tests assures
    /// they are not flipped.
    fn test_x_y_coordinates() {
        let board = Board::deserialize(
            "
  ---  
  ---  
------*
-------
-*-----
  ---  
  ---  
",
        );

        assert_eq!(Occupied, board.at(Position { x: 6, y: 2 }));
        assert_eq!(Empty, board.at(Position { x: 2, y: 6 }));
        assert_eq!(Occupied, board.at(Position { x: 1, y: 4 }));
        assert_eq!(Empty, board.at(Position { x: 4, y: 1 }));
        assert_eq!(NotPartOfBoard, board.at(Position { x: 0, y: 0 }));
    }

    #[test]
    fn failed_move_initial_position_no_piece() {
        let board = Board::deserialize(
            "
  ---  
  ---  
-------
----*--
----*--
  ---  
  ---  
",
        );
        let (from, to) = (Position { x: 4, y: 2 }, Position { x: 4, y: 4 });
        assert_eq!(
            Err(SpacesInvolvedNotCorrect(
                INVALID_MOVE_MESSAGE,
                board.clone(),
                Move { from, to }
            )),
            board.move_piece(from, to)
        );
    }

    #[test]
    fn failed_move_end_position_not_empty() {
        let board = Board::deserialize(
            "
  ---  
  ---  
----*--
----*--
----*--
  ---  
  ---  
",
        );
        let (from, to) = (Position { x: 4, y: 4 }, Position { x: 4, y: 2 });

        assert_eq!(
            Err(SpacesInvolvedNotCorrect(
                INVALID_MOVE_MESSAGE,
                board.clone(),
                Move { from, to }
            )),
            board.move_piece(from, to)
        );
    }

    #[test]
    fn failed_move_skip_position_not_occupied() {
        let board = Board::deserialize(
            "
  ---  
  ---  
-------
-------
----*--
  ---  
  ---  
",
        );
        let (from, to) = (Position { x: 4, y: 4 }, Position { x: 4, y: 2 });

        assert_eq!(
            Err(SpacesInvolvedNotCorrect(
                INVALID_MOVE_MESSAGE,
                board.clone(),
                Move { from, to }
            )),
            board.move_piece(from, to)
        );
    }
}
