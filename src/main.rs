mod board;
mod board_serde;
mod error;
mod find_moves;
mod move_struct;
mod position;
mod solver;
mod stdout_render;

use crate::board::Board;
use crate::position::Position;
use crate::solver::Solver;
use crate::stdout_render::print_board;
use std::time::Instant;

fn main() {
    let b = Board::new();
    print_board(&b);
    let res = b.move_piece(Position { x: 1, y: 3 }, Position { x: 3, y: 3 });
    match res {
        Ok(board) => print_board(&board),
        Err(err) => println!("Error: {:?}", err),
    }

    let board = Board::deserialize(
        "
  ---  
  -*-  
---*---
-------
-------
  ---  
  ---  
",
    );

    print_board(&board);
    let res = board.move_piece(Position { x: 3, y: 1 }, Position { x: 3, y: 3 });
    match res {
        Ok(board) => print_board(&board),
        Err(err) => println!("Error: {:?}", err),
    }

    let mut solver = Solver::new();

    let start = Instant::now();
    let moves = solver.solve().expect("has solution");
    let elapsed = start.elapsed();
    let mut board = Board::new();
    for m in moves {
        print_board(&board);
        board = board.move_piece(m.from, m.to).expect("valid movement");
    }
    print_board(&board);
    println!("Time elapsed: {:.2?}", elapsed);
}
