
use super::super::*;
use super::common::*;

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
    gruvi(iterator, &DIRECTIONS, true)
}
