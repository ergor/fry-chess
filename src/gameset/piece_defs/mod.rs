
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

fn add_vect(origin: &Position, vect: (i32, i32)) -> Option<Position> {
    if let (0, 0) = vect {
        return None;
    }
    let pos = origin.add_vect(vect);
    if Board::within_bounds(&pos) { 
        Some(pos) 
    } else { 
        None
    }
}

pub fn gen_iter(piece: &Piece, board: &Board, vect: (i32, i32)) -> Vec<Board> {
    println!("gen iter");
    if let (0, 0) = vect {
        return Vec::new();
    }

    // generate all new position based on starting position anv vector
    // as long as new position is within bounds
    
    let origin = &piece.position;
    let mut generated = Vec::new();
    let mut new_pos = add_vect(origin, vect);
    
    loop {
        match new_pos {
            Some(pos) => {
                println!("{:?}", pos);
                let mut new_piece = piece.clone();
                let mut new_pieces: Vec<Piece> = board.pieces.iter()
                    .filter(|p| !p.position.cmp(origin) && !p.position.cmp(&pos))
                    .cloned().collect();

                new_pos = add_vect(&pos, vect);
                new_piece.position = pos;
                new_pieces.push(new_piece);

                let new_board = Board::new(new_pieces);
                generated.push(new_board);  
            },
            None => break
        }
    }

    generated
}
