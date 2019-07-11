
use super::super::*;
use super::common::*;

pub const DIRECTIONS: [[Vector; MAX_MOVES]; 4] = [
    ascending_vector!(Vector {x: 1, y: 0}),
    ascending_vector!(Vector {x:-1, y: 0}),
    ascending_vector!(Vector {x: 0, y: 1}),
    ascending_vector!(Vector {x: 0, y:-1}),
];

pub fn generator(board: &Board, piece: &Piece) -> Vec<Position> {
    generate(board, &DIRECTIONS, piece)
}
