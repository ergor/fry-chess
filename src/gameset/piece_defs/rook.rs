
use super::PieceDef;
use super::super::{Vector};
use super::super::board::BoardGenerator;

const DIRS_SZ: usize = 4;
const DIRECTIONS: [Vector; DIRS_SZ] = [
    Vector {x: 1, y: 0},
    Vector {x:-1, y: 0},
    Vector {x: 0, y: 1},
    Vector {x: 0, y:-1},
];

pub fn def() -> PieceDef {
    PieceDef {
        symbol: 'r',
        value: 500,
        vector_iterator,
    }
}

fn vector_iterator(iterator: &mut BoardGenerator) -> Option<Vector> {
    super::repetetive_generator(iterator, &DIRECTIONS)
}
