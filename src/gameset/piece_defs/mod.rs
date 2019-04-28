
pub mod pawn;
pub mod rook;

use super::{Piece, Color, Position, Vector};
use super::board::{Board, BoardGenerator, BoardGeneratorState};

pub struct PieceDef {
    symbol: char,
    value: i32,
    vector_iterator: fn(&mut BoardGenerator) -> Option<Vector>
}

pub fn from_def(def: PieceDef, color: Color, position: Position) -> Piece {
    Piece::new(color, def.symbol, def.value, position, def.vector_iterator)
}

/// out of bounds and phaze-thru-pieces check
fn common_validation(board: &Board, piece: &Piece, landing_sq: Position, direction: Vector) -> bool {
    Board::within_bounds(landing_sq)
            && match board.piece_at(landing_sq - direction) {
                None => true,
                //Some(other_piece) => piece as *const _ == other_piece as *const _,
                Some(some_piece) => piece.is(some_piece)
            }
}

pub fn repetetive_generator(
        iterator: &mut BoardGenerator, 
        directions: &[Vector]) -> Option<Vector> {

    let board = iterator.basis_board;
    let piece = iterator.piece;
    let (i, vect) = match iterator.state {
        BoardGeneratorState::Next(i) => {
            if i == directions.len() { return None; }
            (i, directions[i])
        },
        BoardGeneratorState::Accumulate(i, acc) => (i, acc.add(directions[i]))
    };

    let dir_vect = directions[i];
    let landing_sq = piece.position + vect;
    if common_validation(board, piece, landing_sq, dir_vect)
        && match board.piece_at(landing_sq) {
            None => true,
            Some(other_piece) => piece.is_enemy_of(other_piece),
            } {
        iterator.state = BoardGeneratorState::Accumulate(i, vect);
        Some(vect)
    } else {
        iterator.state = BoardGeneratorState::Next(i + 1);
        return repetetive_generator(iterator, directions);
    }
}
