
pub mod pawn;
pub mod rook;

use super::{Piece, Board, Position};

pub fn gen_iter(piece: &Box<Piece>, board: &Board, vect: (i32, i32)) -> Vec<Board> {
    let piece = piece.piece();
    let pieces: Vec<&Box<Piece>> = board.pieces.iter()
        .filter(|p| !p.piece().position.cmp(&piece.position))
        .collect();

    let mut generated = Vec::new();
    
    let mut new_pos = piece.position.add_vect(vect);
    while Board::within_bounds(&new_pos) {
        let new_pieces = pieces.to_owned();
        let new_board = Board::new(new_pieces);
        generated.push(new_board);
    }

    generated
}