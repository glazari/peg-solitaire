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
#[derive(Debug, PartialEq)]
pub struct Board {
    // TODO: pub(create) is used to test the to_str function, there is probably a a better way of
    // doing this
    pub(crate) board: [[Space; 7]; 7],
}

use Space::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum Space {
    Empty,
    Occupied,
    NotPartOfBoard,
}

/// Describes a position on the board.
type Position = (u32, u32);

/*
 * Board Game methods
 *
 */
impl Board {
    pub fn move_piece(&self, from: Position, to: Position) -> Board {
        Board::new()
    }
}

/// Serialize and deserialize Board
impl Board {
    pub(crate) fn serialize(&self) -> String {
        let mut b_str = String::new();
        b_str.push_str("\n");
        for i in 0..7 {
            for j in 0..7 {
                match self.board[i][j] {
                    NotPartOfBoard => b_str.push_str(" "),
                    Empty => b_str.push_str("-"),
                    Occupied => b_str.push_str("*"),
                }
            }
            b_str.push_str("\n");
        }
        b_str
    }
    fn deserialize(board_str: &str) -> Board {
        let mut board = Board::new();
        let (mut i, mut j) = (0, 0);
        for n in 1..board_str.len() {
            let char = board_str.chars().nth(n);
            match char {
                None => return board,
                Some(char) => match char {
                    ' ' => {
                        board.board[i][j] = NotPartOfBoard;
                        i += 1;
                    }
                    '-' => {
                        board.board[i][j] = Empty;
                        i += 1;
                    }
                    '*' => {
                        board.board[i][j] = Occupied;
                        i += 1;
                    }
                    '\n' => {
                        i = 0;
                        j += 1;
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
        let non_full_board = Board {
            board: [
                [NotPartOfBoard,  NotPartOfBoard, Occupied, Occupied, Occupied, NotPartOfBoard,  NotPartOfBoard],
                [NotPartOfBoard,  NotPartOfBoard, Occupied, Empty,    Occupied, NotPartOfBoard,  NotPartOfBoard],
                [Occupied,        Empty,          Empty,    Occupied, Occupied, Occupied,        Occupied],
                [Occupied,        Occupied,       Occupied, Occupied, Occupied, Occupied,        Occupied],
                [Occupied,        Occupied,       Occupied, Occupied, Occupied, Occupied,        Occupied],
                [NotPartOfBoard,  NotPartOfBoard, Occupied, Occupied, Occupied, NotPartOfBoard,  NotPartOfBoard],
                [NotPartOfBoard,  NotPartOfBoard, Occupied, Occupied, Occupied, NotPartOfBoard,  NotPartOfBoard],
            ]
        };
        let expected = "
  ***  
  *-*  
*--****
*******
*******
  ***  
  ***  
";
        assert_eq!(expected, non_full_board.serialize());
    }
}
