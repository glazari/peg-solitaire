//! Brute force solver

use crate::board::Board;
use crate::move_struct::Move;

pub struct Solver {
    board: Board,
    count: i32,
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            board: Board::new(),
            count: 0,
        }
    }
    pub fn from_str(b_str: &str) -> Solver {
        Solver {
            board: Board::deserialize(b_str),
            count: 0,
        }
    }
    pub fn solve(&mut self) -> Option<Vec<Move>> {
        let mut solution = self.solve_board(self.board.clone())?;
        solution.reverse();
        Some(solution)
    }
    fn solve_board(&mut self, board: Board) -> Option<Vec<Move>> {
        //println!("Enter board: {}", board.serialize());
        let moves = board.find_moves();
        if moves.len() == 0 && board.find_pieces().len() == 1 {
            return Some(Vec::with_capacity(32));
        }
        self.count += 1;
        //println!("moves found:\n {:?}", moves);

        for m in moves {
            let new_board = board
                .move_piece(m.from, m.to)
                .expect("all moves should be valid at this point");
            let option_moves = self.solve_board(new_board);
            match option_moves {
                Some(mut winning_moves) => {
                    winning_moves.push(m);
                    return Some(winning_moves);
                }
                None => continue,
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::position::Position;
    #[test]
    fn solve_simple_game() {
        let mut solver = Solver::from_str(
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

        let expected = vec![Move {
            from: Position { x: 4, y: 3 },
            to: Position { x: 4, y: 5 },
        }];

        let got = solver.solve().expect("There is a valida solution here");

        assert_eq!(expected, got);
    }

    #[test]
    fn solve_tree_piece_simple_game() {
        let mut solver = Solver::from_str(
            "
  ---  
  ---  
---*---
----*--
----*--
  ---  
  ---  
",
        );

        let expected = vec![
            Move {
                from: Position { x: 4, y: 4 },
                to: Position { x: 4, y: 2 },
            },
            Move {
                from: Position { x: 3, y: 2 },
                to: Position { x: 5, y: 2 },
            },
        ];

        let got = solver.solve().expect("There is a valida solution here");

        assert_eq!(expected, got);
    }
    #[test]
    fn solve_four_piece_simple_game() {
        let mut solver = Solver::from_str(
            "
  ---  
  ---  
-*-*---
----*--
----*--
  ---  
  ---  
",
        );

        let expected = vec![
            Move {
                from: Position { x: 4, y: 4 },
                to: Position { x: 4, y: 2 },
            },
            Move {
                from: Position { x: 4, y: 2 },
                to: Position { x: 2, y: 2 },
            },
            Move {
                from: Position { x: 1, y: 2 },
                to: Position { x: 3, y: 2 },
            },
        ];

        let got = solver.solve().expect("There is a valida solution here");

        assert_eq!(expected, got);
    }

    #[test]
    #[ignore = "solves all but does not have the expected steps"]
    fn solve_all() {
        let mut solver = Solver::from_str(
            "
  ***  
  ***  
*******
***-***
*******
  ***  
  ***  
",
        );

        let expected = vec![
            Move {
                from: Position { x: 4, y: 4 },
                to: Position { x: 4, y: 2 },
            },
            Move {
                from: Position { x: 3, y: 2 },
                to: Position { x: 5, y: 2 },
            },
        ];

        let got = solver.solve().expect("There is a valida solution here");

        println!("number of moves {}", got.len());
        println!("number moves {}", solver.count);

        assert_eq!(expected, got);
    }
}
