use super::super::*;

pub const MAX_MOVES: usize = 7; // why can't i use this in the macros?

/// out of bounds and phaze-thru-pieces check
pub fn boundaries_ok(board: &Board, piece: &Piece, landing_sq: Position, direction: Vector) -> bool {
    Board::within_bounds(landing_sq)
            && match board.piece_at(landing_sq - direction) {
                None => true,
                Some(some_piece) => piece.is(some_piece)
            }
}

pub fn generate(board: &Board, moves: &[[Vector; MAX_MOVES]], moving_piece: &Piece) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();

    for direction in moves {
        for vector in direction {
            // go to next direction if we find padding
            if vector.x == 0 && vector.y == 0 {
                break;
            }
            
            let vector = *vector;
            let new_pos = moving_piece.position + vector;
            if Board::within_bounds(new_pos) {
                let piece = board.piece_at(new_pos);
                if piece.is_some() {
                    if piece.unwrap().is_enemy_of(moving_piece) {
                        positions.push(new_pos); // capture enemy
                        break; // but don't move further in this direction
                    } else {
                        break; // friendly piece; can't go here nor further
                    }
                }
                positions.push(new_pos);
            }
        }
    }

    positions
}

/**
 * IT IS IMPORTANT THAT THE FACTOR == INDEX
 */
#[macro_export]
macro_rules! ascending_vector {
    ($v:expr) => {
        {
            const VECT: [Vector; 7 /*MAX_MOVES*/] = [
                Vector {x: 1 * $v.x, y: 1 * $v.y},
                Vector {x: 2 * $v.x, y: 2 * $v.y},
                Vector {x: 3 * $v.x, y: 3 * $v.y},
                Vector {x: 4 * $v.x, y: 4 * $v.y},
                Vector {x: 5 * $v.x, y: 5 * $v.y},
                Vector {x: 6 * $v.x, y: 6 * $v.y},
                Vector {x: 7 * $v.x, y: 7 * $v.y},
            ];
            VECT
        }
    };
}

#[macro_export]
macro_rules! pad_vector {
    ($v:expr) => {
        {
            const VECT: [Vector; 7 /*MAX_MOVES*/] = [
                Vector {x: $v.x, y: $v.y},
                Vector {x: 0, y: 0},
                Vector {x: 0, y: 0},
                Vector {x: 0, y: 0},
                Vector {x: 0, y: 0},
                Vector {x: 0, y: 0},
                Vector {x: 0, y: 0},
            ];
            VECT
        }
    };
}