
use super::PieceDef;
use super::super::{Piece, Board, Position, Vector2D, BoardGenerator, GeneratorState};

pub fn def() -> PieceDef {
    PieceDef {
        symbol: 'r',
        value: 500,
        generator: generate
    }
}

fn generate(origin: Position, board: &Board) -> Board {

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

const directions: [Vector2D; 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

// (position, has_next)
fn next_vector(iterator: &BoardGenerator) -> (Option<Vector2D>, bool) {
    match iterator.state {
        Next(i) => {
        },
        Continue(i, pos) => {
        },
    }
}