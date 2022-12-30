//! Board serializatin strategy
//!
//! Allos for Board to be serialized to string and deserialized from
//! string.

use crate::board::Board;
use crate::board::Space::*;
use crate::position::Position;

/// Serialize and deserialize Board
impl Board {
    pub(crate) fn serialize(&self) -> String {
        let mut b_str = String::new();
        b_str.push_str("\n");
        for y in 0..7 {
            for x in 0..7 {
                match self.at(Position { x, y }) {
                    NotPartOfBoard => b_str.push_str(" "),
                    Empty => b_str.push_str("-"),
                    Occupied => b_str.push_str("*"),
                }
            }
            b_str.push_str("\n");
        }
        b_str
    }
    pub(crate) fn deserialize(board_str: &str) -> Board {
        let mut board = Board::new();
        let (mut x, mut y) = (0, 0);
        for n in 1..board_str.len() {
            let char = board_str.chars().nth(n);
            match char {
                None => return board,
                Some(char) => match char {
                    ' ' => {
                        board.set(Position { x, y }, NotPartOfBoard);
                        x += 1;
                    }
                    '-' => {
                        board.set(Position { x, y }, Empty);
                        x += 1;
                    }
                    '*' => {
                        board.set(Position { x, y }, Occupied);
                        x += 1;
                    }
                    '\n' => {
                        x = 0;
                        y += 1;
                    }
                    _ => panic!("Invalid char '{}'", char),
                },
            }
            if char.is_none() {
                return board;
            }
        }
        board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_board_serialize() {
        let full_board = Board::new();
        let expected = "
  ***  
  ***  
*******
***-***
*******
  ***  
  ***  
";
        assert_eq!(expected, full_board.serialize());
        assert_eq!(full_board, Board::deserialize(expected));
    }
    #[test]
    fn non_full_board_serialize() {
        #[rustfmt::skip]
        let non_full_board = Board::from_array(
             [
                [NotPartOfBoard,  NotPartOfBoard, Occupied, Occupied, Occupied, NotPartOfBoard,  NotPartOfBoard],
                [NotPartOfBoard,  NotPartOfBoard, Occupied, Empty,    Occupied, NotPartOfBoard,  NotPartOfBoard],
                [Occupied,        Empty,          Empty,    Occupied, Occupied, Occupied,        Occupied],
                [Occupied,        Occupied,       Occupied, Occupied, Occupied, Occupied,        Occupied],
                [Occupied,        Occupied,       Occupied, Occupied, Occupied, Occupied,        Occupied],
                [NotPartOfBoard,  NotPartOfBoard, Occupied, Occupied, Occupied, NotPartOfBoard,  NotPartOfBoard],
                [NotPartOfBoard,  NotPartOfBoard, Occupied, Occupied, Occupied, NotPartOfBoard,  NotPartOfBoard],
            ]
        );
        let expected = "
  ***  
  *-*  
*--****
*******
*******
  ***  
  ***  
";
        let got = non_full_board.serialize();
        assert_eq!(expected, got, "\nExpected: {}\nGot: {}", expected, got);
        assert_eq!(non_full_board, Board::deserialize(expected));
    }
}
