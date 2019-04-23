
use super::PieceDef;
use super::super::{Piece, Board, Position};

pub fn def() -> PieceDef {
    PieceDef {
        symbol: 'r',
        value: 500,
        generator: generate
    }
}

fn generate(origin: Position, board: &Board) -> Vec<Board> {
    super::gen_iter(origin, board, vec![(1, 0), (-1, 0), (0, 1), (0, -1)])
}
