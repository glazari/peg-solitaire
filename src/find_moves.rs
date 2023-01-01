//! implements the function to find moves on the board

use crate::board::Board;
use crate::board::Space::*;
use crate::move_struct::Move;
use crate::position::Position;

impl Board {
    pub fn find_moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        let pieces = self.find_pieces();
        for piece in pieces.iter() {
            for neighbor in self.neighbors(*piece).iter() {
                let m = Move {
                    from: *piece,
                    to: *neighbor,
                };
                if self.valid_move(m).is_ok() {
                    moves.push(m);
                }
            }
        }
        moves
    }

    fn neighbors(&self, p: Position) -> Vec<Position> {
        let mut neighbors = vec![];
        if p.x > 1 {
            neighbors.push(Position { x: p.x - 2, y: p.y });
        }
        if p.x < 5 {
            neighbors.push(Position { x: p.x + 2, y: p.y });
        }
        if p.y > 1 {
            neighbors.push(Position { x: p.x, y: p.y - 2 });
        }
        if p.y < 5 {
            neighbors.push(Position { x: p.x, y: p.y + 2 });
        }
        neighbors
    }

    fn find_pieces(&self) -> Vec<Position> {
        let mut pos = vec![];
        for y in 0..7 {
            for x in 0..7 {
                let p = Position { x, y };
                match self.at(p) {
                    Occupied => pos.push(p),
                    _ => (),
                }
            }
        }
        pos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simples_move_list() {
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

        let mut expected = vec![
            Move {
                from: Position { x: 4, y: 4 },
                to: Position { x: 4, y: 2 },
            },
            Move {
                from: Position { x: 4, y: 3 },
                to: Position { x: 4, y: 5 },
            },
        ];

        let mut got = board.find_moves();

        // the order does not matter
        expected.sort();
        got.sort();

        assert_eq!(expected, got)
    }

    #[test]
    fn three_piece_move_list() {
        let board = Board::deserialize(
            "
  ---  
  ---  
-------
----*--
----**-
  ---  
  ---  
",
        );

        let mut expected = vec![
            Move {
                from: Position { x: 4, y: 4 },
                to: Position { x: 4, y: 2 },
            },
            Move {
                from: Position { x: 4, y: 3 },
                to: Position { x: 4, y: 5 },
            },
            Move {
                from: Position { x: 4, y: 4 },
                to: Position { x: 6, y: 4 },
            },
            Move {
                from: Position { x: 5, y: 4 },
                to: Position { x: 3, y: 4 },
            },
        ];

        let mut got = board.find_moves();

        // the order does not matter
        expected.sort();
        got.sort();

        assert_eq!(expected, got)
    }
    #[test]
    fn simplest_position_list() {
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

        let mut expected = vec![Position { x: 4, y: 4 }, Position { x: 4, y: 3 }];

        let mut got = board.find_pieces();

        // the order does not matter
        expected.sort();
        got.sort();

        assert_eq!(expected, got)
    }
}
