
use super::super::*;
use super::common::*;

const DIRS_SZ: usize = 8;
const DIRECTIONS: [Vector; DIRS_SZ] = [
    Vector {x: 1, y: 2},
    Vector {x:-1, y: 2},
    Vector {x: 1, y:-2},
    Vector {x:-1, y:-2},
    Vector {x: 2, y: 1},
    Vector {x:-2, y: 1},
    Vector {x: 2, y:-1},
    Vector {x:-2, y:-1},
];

pub fn def() -> PieceDef {
    PieceDef {
        symbol: 'n',
        value: 300,
        vector_iterator,
    }
}

fn vector_iterator(iterator: &mut BoardGenerator) -> Option<Vector> {
    gruvi(iterator, &DIRECTIONS, false)
}
