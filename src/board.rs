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
pub struct Board {
    // TODO: pub(create) is used to test the to_str function, there is probably a a better way of
    // doing this
    pub(crate) board: [[Space; 7]; 7],
}

use Space::*;

#[derive(Copy, Clone)]
pub(crate) enum Space {
    Empty,
    Occupied,
    NotPartOfBoard,
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
