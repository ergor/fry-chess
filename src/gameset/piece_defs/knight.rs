
use super::super::*;
use super::common::*;

const DIRS_SZ: usize = 8;
const DIRECTIONS: [[Vector; MAX_MOVES]; DIRS_SZ] = [
    pad_vector!(Vector {x: 1, y: 2}),
    pad_vector!(Vector {x:-1, y: 2}),
    pad_vector!(Vector {x: 1, y:-2}),
    pad_vector!(Vector {x:-1, y:-2}),
    pad_vector!(Vector {x: 2, y: 1}),
    pad_vector!(Vector {x:-2, y: 1}),
    pad_vector!(Vector {x: 2, y:-1}),
    pad_vector!(Vector {x:-2, y:-1}),
];

pub fn def() -> PieceDef {
    PieceDef {
        symbol: 'n',
        value: 300,
        generator,
    }
}

fn generator(board: &Board, piece: &Piece) -> Vec<Position> {
    generate(&DIRECTIONS, piece)
}
