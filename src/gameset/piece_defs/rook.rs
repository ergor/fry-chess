
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

    let moving_piece: &Piece = board.piece_at(origin)
        .unwrap();

    let condition = |landing_sq: Position| { 
        let opt_piece = board.piece_at(landing_sq);
        match opt_piece {
            None => true,
            Some(piece) => piece.is_enemy(moving_piece.color)
        }
    };
    let moves = super::gen_iter(origin, vec![(1, 0), (-1, 0), (0, 1), (0, -1)], &condition);
    super::gen_fixed(origin, board, moves)
}
