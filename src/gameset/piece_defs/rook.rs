
use super::PieceDef;
use super::super::{Board, Piece, Vector, BoardGenerator, GeneratorState};

const DIRS_SZ: usize = 4;
const DIRECTIONS: [Vector; DIRS_SZ] = [
    Vector {x: 1, y: 0},
    Vector {x:-1, y: 0},
    Vector {x: 0, y: 1},
    Vector {x: 0, y:-1},
];

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

fn vector_iterator(iterator: &mut BoardGenerator) -> Option<Vector> {

    fn is_legal(board: &Board, piece: &Piece, vect: Vector) -> bool {
        println!("trying {:?} + {:?}", piece.position, vect);
        let landing_sq = piece.position + vect;
        Board::within_bounds(landing_sq) 
            && match board.piece_at(landing_sq) {
                None => true,
                Some(other_piece) => piece.is_enemy(other_piece.color),
            }
    }

    let board = iterator.basis_board;
    let piece = iterator.piece;
    match iterator.state {
        GeneratorState::Next(i) => {
            if i == DIRS_SZ {
                return None;
            }
            let vect = DIRECTIONS[i];
            if is_legal(board, piece, vect) {
                iterator.state = GeneratorState::Continue(i, vect);
                return Some(vect)
            } else {
                iterator.state = GeneratorState::Next(i + 1);
                return vector_iterator(iterator);
            }
        },

        GeneratorState::Continue(i, acc) => {
            let vect = acc.add(DIRECTIONS[i]);
            if is_legal(board, piece, vect) {
                iterator.state = GeneratorState::Continue(i, vect);
                return Some(vect)
            } else {
                iterator.state = GeneratorState::Next(i + 1);
                return vector_iterator(iterator);
            }
        },
    };
}