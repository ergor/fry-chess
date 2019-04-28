
pub mod pawn;
pub mod rook;
pub mod knight;

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

/**
 * the GRand Unified Vector Iterator
 */
pub fn gruvi(iterator: &mut BoardGenerator,
        directions: &[Vector], accumulate: bool) -> Option<Vector> {

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
        iterator.state =
            if accumulate { BoardGeneratorState::Accumulate(i, vect) } 
            else { BoardGeneratorState::Next(i + 1) };
        Some(vect)
    } else {
        iterator.state = BoardGeneratorState::Next(i + 1);
        return gruvi(iterator, directions, accumulate);
    }
}
