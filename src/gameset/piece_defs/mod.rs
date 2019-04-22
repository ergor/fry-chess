
pub mod pawn;
pub mod rook;

use super::{Piece, Board, Position, Color};
use std::collections::HashMap;

pub struct PieceDef {
    symbol: char,
    value: i32,
    generator: fn(&Piece, &Board) -> Vec<Board>
}

pub fn new(def: PieceDef, color: Color) -> Piece {
    Piece::new(
        color,
        def.symbol,
        def.value,
        def.generator
    )
}

fn add_vect(origin: Position, vect: (i32, i32)) -> Option<Position> {
    if let (0, 0) = vect {
        return None;
    }
    let pos = origin.add_vect(vect);
    if Board::within_bounds(pos) { 
        Some(pos) 
    } else { 
        None
    }
}

pub fn gen_iter(origin: Position, board: &Board, vect: (i32, i32)) -> Vec<Board> {
    println!("gen iter");
    if let (0, 0) = vect {
        return Vec::new();
    }

    // generate all new position based on starting position anv vector
    // as long as new position is within bounds
    
    let mut generated = Vec::new();
    let mut new_pos = add_vect(origin, vect);
    
    loop {
        match new_pos {
            Some(pos) => {
                println!("{:?}", pos);
                let mut new_piece: Piece = board.piece_at(origin).unwrap().clone();
                let mut new_pieces = HashMap::new();
                for (k, v) in board.pieces {
                    if !k.cmp(origin) && !k.cmp(pos) {
                        new_pieces.insert(k, v);
                    }
                }

                new_pieces.insert(pos, new_piece);

                let new_board = Board::new(new_pieces);
                generated.push(new_board);
                new_pos = add_vect(pos, vect);
            },
            None => break
        }
    }

    generated
}
