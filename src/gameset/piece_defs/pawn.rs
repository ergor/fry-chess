
use super::PieceDef;
use super::super::{Piece, Board};

pub fn def() -> PieceDef {
    PieceDef {
        symbol: 'p',
        value: 100,
        generator
    }
}

fn generator(piece: &Piece, board: &Board) -> Vec<Board> {
    Vec::new()
}