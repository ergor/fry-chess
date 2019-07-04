
use super::super::*;
use super::common::*;

// WHAT ABOUT USING A MACRO TO GENERATE ALL ABOSULUTE MOVEMENT VECTORS?
// then there is only one type of movement vector

pub const DIRECTIONS: [[Vector; MAX_MOVES]; 4] = [
    ascending_vector!(Vector {x: 1, y: 0}),
    ascending_vector!(Vector {x:-1, y: 0}),
    ascending_vector!(Vector {x: 0, y: 1}),
    ascending_vector!(Vector {x: 0, y:-1}),
];

pub fn def() -> PieceDef {
    PieceDef {
        symbol: 'r',
        value: 500,
        generator
    }
}

fn generator(board: &Board, piece: &Piece) -> Vec<Position> {
    generate(&DIRECTIONS, piece)
}
