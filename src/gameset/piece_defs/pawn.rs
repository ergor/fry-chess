
use super::PieceDef;
use super::super::{Vector, BoardGenerator};

pub fn def() -> PieceDef {
    PieceDef {
        symbol: 'p',
        value: 100,
        vector_iterator,
    }
}

fn vector_iterator(iterator: &BoardGenerator) -> Option<Vector> {
    None
}