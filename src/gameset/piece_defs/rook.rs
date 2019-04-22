
use super::PieceDef;
use super::super::{Piece, Board};

pub fn def() -> PieceDef {
    PieceDef {
        symbol: 'r',
        value: 500,
        generator
    }
}

fn generator(piece: &Piece, board: &Board) -> Vec<Board> {
    Vec::new()
}
