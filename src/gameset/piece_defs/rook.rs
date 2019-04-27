
use super::PieceDef;
use super::super::{Board, Piece, Vector, BoardGenerator, GeneratorState};

pub fn def() -> PieceDef {
    PieceDef {
        symbol: 'r',
        value: 500,
        vector_iterator,
    }
}

/*
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
*/

const DIRS_SZ: usize = 4;
const directions: [Vector; DIRS_SZ] = [
    Vector {x: 1, y: 0},
    Vector {x:-1, y: 0},
    Vector {x: 0, y: 1},
    Vector {x: 0, y:-1},
];

fn vector_iterator(iterator: &BoardGenerator) -> Option<Vector> {
    fn is_legal(piece: &Piece, vect: Vector) -> bool {
        Board::within_bounds(piece.position.add_vector(vect))
    }

    let piece = iterator.piece;
    let delta_vect = match iterator.state {

        GeneratorState::Next(i) => {
            if i == DIRS_SZ {
                return None;
            }
            let vect = directions[i];
            iterator.state = GeneratorState::Next(i + 1);
            if is_legal(piece, vect) {
                return Some(vect)
            } else {
                return vector_iterator(iterator);
            }
        },

        GeneratorState::Continue(i, acc) => {
            let vect = acc.add(directions[i]);
            if is_legal(piece, vect) {
                iterator.state = GeneratorState::Continue(i, vect);
                return Some(vect)
            } else {
                iterator.state = GeneratorState::Next(i + 1);
                return vector_iterator(iterator);
            }
        },
    };
}