
use super::PieceDef;
use super::super::{Board, Color, Vector, BoardGenerator, GeneratorState};

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
        GeneratorState::Next(i) => {
            let vector = match piece.color {
                Color::WHITE => DIRECTIONS[i],
                Color::BLACK => -DIRECTIONS[i],
            };

            iterator.state = GeneratorState::Next(i + 1);

            match i {
                0 => { // forward 1
                    let landing_sq = piece.position + vector;
                    if Board::within_bounds(landing_sq)
                        && board.piece_at(landing_sq).is_none() {
                        return Some(vector)
                    } else {
                        return vector_iterator(iterator);
                    }
                },
                1 => { // forward 2
                    return None;
                },
                2 => { // attack left
                    return None;
                },
                3 => { // attack right
                    return None;
                },
                _ => return None,
            }
        },
        _ => panic!("generator state not defined for pawn"),
    }
}