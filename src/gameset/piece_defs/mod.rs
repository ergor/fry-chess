
pub mod pawn;
pub mod rook;

use super::{Piece, Board, Position, Color};
use std::collections::HashMap;

pub struct PieceDef {
    symbol: char,
    value: i32,
    generator: fn(Position, &Board) -> Vec<Board>
}

pub fn new(def: PieceDef, color: Color) -> Piece {
    Piece::new(
        color,
        def.symbol,
        def.value,
        def.generator
    )
}

/**
 * Returns the landing square if the position is reachable,
 * or None if unreachable or movement vector i null vector.
 * 
 * # Arguments
 * * `origin` - the current position of the piece
 * * `vect` - the vector to move the piece along
 * * `condition` - closure that must return whether the landing square is legal.
 */
fn apply_vector(origin: Position, vect: (i32, i32), condition: &Fn(Position) -> bool) -> Option<Position> {
    if let (0, 0) = vect {
        return None;
    }

    let landing_sq = origin.add_vect(vect);

    if Board::within_bounds(landing_sq) && condition(landing_sq) {
        Some(landing_sq)
    } else {
        None
    }
}

/**
 * Generate boards by repeatedly adding a movement vector to the position
 * of the piece we're moving, until it cannot be legally moved anymore.
 * Generation starts from the piece's origin position.
 * Can generate for any number of movement vectors.
 * 
 * # Arguments
 * * `origin` - the starting position of the piece
 * * `basis_board` - the board on which the piece stands
 * * `vects` - the movement vectors
 */
pub fn gen_iter(origin: Position, basis_board: &Board, vects: Vec<(i32, i32)>) -> Vec<Board> {

    let moving_piece: &Piece = basis_board.piece_at(origin)
        .unwrap();

    let condition = |landing_sq: Position| { 
        let opt_piece = basis_board.piece_at(landing_sq);
        match opt_piece {
            None => true,
            Some(piece) => piece.is_enemy(moving_piece.color)
        }
    };

    let mut generated_boards = Vec::new();

    for vect in vects {
        if let (0, 0) = vect {
            continue;
        }

        let mut opt_landing_sq = apply_vector(origin, vect, &condition);

        loop {
            match opt_landing_sq {
                None => break,
                Some(landing_sq) => {
                    println!("{:?}", landing_sq);

                    let moving_piece: Piece = moving_piece.clone();

                    let mut pieces: HashMap<Position, Piece> = HashMap::new();
                    for (pos, piece) in &basis_board.pieces {
                        if !pos.cmp(origin) { // dont clone the moving piece yet
                            pieces.insert(*pos, piece.clone());
                        }
                    }

                    pieces.insert(landing_sq, moving_piece);

                    generated_boards.push(Board::new(pieces));

                    // advance the loop
                    opt_landing_sq = apply_vector(landing_sq, vect, &condition);
                }
            }
        }
    }

    generated_boards
}

// idea:
// one function which just applies every vector as is. this is the fixed one (knights, pawns, etc)
// one function which generates legal, fixed, movement vectors iteratively from direction vectors.
// then calls the fixed vector function to apply them all. (rooks, bishops, etc)

pub fn gen_fixed(origin: Position, basis_board: &Board, vects: Vec<(i32, i32)>, condition: &Fn(Position) -> bool) -> Vec<Board> {
    let mut generated_boards = Vec::new();

    for vect in vects {
        if let (0, 0) = vect {
            continue;
        }

        let mut opt_landing_sq = apply_vector(origin, vect, condition);

        match opt_landing_sq {
            None => continue,
            Some(landing_sq) => {
                println!("{:?}", landing_sq);

                let moving_piece: Piece = basis_board.piece_at(origin)
                    .unwrap()
                    .clone();

                let mut pieces: HashMap<Position, Piece> = HashMap::new();
                for (pos, piece) in &basis_board.pieces {
                    if !pos.cmp(origin) { // dont clone the moving piece yet
                        pieces.insert(*pos, piece.clone());
                    }
                }

                pieces.insert(landing_sq, moving_piece);

                generated_boards.push(Board::new(pieces));
            }
        }
    }

    generated_boards
}
