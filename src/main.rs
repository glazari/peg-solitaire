mod stdout_render;
mod board;

use crate::stdout_render::print_board;
use crate::board::Board;

fn main() {
    let b = Board::new();
    print_board(b);
    let a = [[0; 7]; 7];
    println!("{:?}", a)
}


