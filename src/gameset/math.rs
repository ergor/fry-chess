
use super::{Vector, Position};

pub fn line(origin: Position, direction: Vector) -> impl Fn(i32) -> Position {
    move | step | { origin + (direction * step) }
}

