
use super::PieceDef;
use super::super::{Color, Vector};
use super::super::board::{BoardGenerator, BoardGeneratorState};

const DIRS_SZ: usize = 4;
const DIRECTIONS: [Vector; DIRS_SZ] = [
    Vector {x: 0, y:-1},
    Vector {x: 0, y:-2},
    Vector {x:-1, y:-1},
    Vector {x: 1, y:-1},
];

pub fn def() -> PieceDef {
    PieceDef {
        symbol: 'p',
        value: 100,
        vector_iterator,
    }
}

fn vector_iterator(iterator: &mut BoardGenerator) -> Option<Vector> {

    let board = iterator.basis_board;
    let piece = iterator.piece;

    match iterator.state {
        BoardGeneratorState::Next(i) => {
            let vector = match piece.color {
                Color::WHITE => DIRECTIONS[i],
                Color::BLACK => -DIRECTIONS[i],
            };

            iterator.state = BoardGeneratorState::Next(i + 1);
            let landing_sq = piece.position + vector;
            let partially_valid = super::common_validation(board, piece, landing_sq, vector);
            match i {
                0 => { // forward 1
                    if partially_valid && board.piece_at(landing_sq).is_none() {
                        Some(vector)
                    } else {
                        return vector_iterator(iterator);
                    }
                },
                1 => { // forward 2
                    if partially_valid
                        && board.piece_at(piece.position + DIRECTIONS[0]).is_none()
                        && board.piece_at(landing_sq).is_none() {
                        Some(vector)
                    } else {
                        return vector_iterator(iterator);
                    }
                },
                2 | 3 => { // attacks
                    if partially_valid && match board.piece_at(landing_sq) {
                            None => false,
                            Some(other_piece) => piece.is_enemy_of(other_piece)
                        } {
                        Some(vector)
                    } else if i < DIRS_SZ-1 {
                        return vector_iterator(iterator);
                    } else {
                        None
                    }
                },
                _ => None,
            }
        },
        _ => panic!("generator state not defined for pawn"),
    }
}
