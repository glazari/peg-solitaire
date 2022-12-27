//! Render Board to standard out
//!
//! This module is responsible for rendering the board to standard out
//! with a simple ASCII representation. The reason for keeping this
//! separate from the main board definition is that we could have many
//! possible representations of the board. Maybe in the future we'll add
//! a GUI representation of it.

use crate::board::Board;
use crate::board::Space::*;

/// this is a comment about the print function
pub fn print_board(b: Board) {
    println!("{}", b.to_str())
}

impl Board {
    fn to_str(&self) -> String {
        let mut b_str = String::new();
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn full_board_str() {
        let full_board = Board::new();
        let expected = "  ***  
  ***  
*******
***-***
*******
  ***  
  ***  
";
        assert_eq!(expected, full_board.to_str());
    }
    #[test]
    fn non_full_board() {
        let full_board = Board::new();
        let expected = "  ***  
  ***  
*******
***-***
*******
  ***  
  ***  
";
        assert_eq!(expected, full_board.to_str());
    }
}
