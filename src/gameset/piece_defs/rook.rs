
use super::PieceDef;
use super::super::{Piece, Board};

pub fn def() -> PieceDef {
    PieceDef {
        symbol: 'r',
        value: 500,
        generator: generate
    }
}

fn generate(piece: &Piece, board: &Board) -> Vec<Board> {
    super::gen_iter(piece, board, (1, 0))
}
