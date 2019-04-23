
use super::PieceDef;
use super::super::{Piece, Board, Position};

pub fn def() -> PieceDef {
    PieceDef {
        symbol: 'p',
        value: 100,
        generator: generate
    }
}

fn generate(origin: Position, board: &Board) -> Vec<Board> {
    Vec::new()
}