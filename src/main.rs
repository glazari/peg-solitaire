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
use std::time::Duration;
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

    let board = Board::new();
    let mut count = 0;
    let mut sum = Duration::new(0, 0);
    for _i in 1..1000 {
        let start = Instant::now();
        let moves = solver.solve().expect("has solution");
        let elapsed = start.elapsed();
        count += 1;
        sum += elapsed;
        println!("Time elapsed: {:.2?}, moves {}", elapsed, moves.len());
    }
    //for m in moves {
    //    print_board(&board);
    //    board = board.move_piece(m.from, m.to).expect("valid movement");
    // }
    print_board(&board);
    println!("Avg Time elapsed: {:.2?}", sum / count);
}
