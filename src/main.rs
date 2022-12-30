mod board;
mod board_serde;
mod error;
mod position;
mod stdout_render;

use crate::board::Board;
use crate::position::Position;
use crate::stdout_render::print_board;

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
}
