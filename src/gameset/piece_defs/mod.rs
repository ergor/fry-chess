
pub mod pawn;
pub mod rook;

use super::{Piece, Board, Position, Color};

pub struct PieceDef {
    symbol: char,
    value: i32,
    generator: fn(&Piece, &Board) -> Vec<Board>
}

pub fn new(def: PieceDef, color: Color, position: Position) -> Piece {
    Piece::new(
        color,
        def.symbol,
        def.value,
        position,
        def.generator
    )
}

pub fn gen_iter(piece: &Piece, board: &Board, vect: (i32, i32)) -> Vec<Board> {
    if let (0, 0) = vect {
        return Vec::new();
    }

    let pieces: Vec<&Piece> = board.pieces.iter()
        .filter(|p| !p.position.cmp(&piece.position))
        .collect();

    // generate all new position based on starting position anv vector
    // as long as new position is within bounds
    let starting_pos = &piece.position;
    //let ps = ()


    let mut generated = Vec::new();
    
    let mut new_pos = piece.position.add_vect(vect);
    //while Board::within_bounds(&new_pos) {
    //    let new_pieces: Vec<Piece> = pieces.to_owned();
    //    let new_board = Board::new(new_pieces);
    //    generated.push(new_board);
    //}

    generated
}
