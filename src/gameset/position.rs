
use std::ops;
use super::Vector;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position {x, y}
    }

    pub fn add_vector(position: Position, vector: Vector) -> Position {
        let x = position.x as i32 + vector.x;
        let y = position.y as i32 + vector.y;
        Position {
            x: x as usize,
            y: y as usize,
        }
    }
}

impl ops::Add<Vector> for Position {
    type Output = Position;

    fn add(self, rhs: Vector) -> Position {
        Position::add_vector(self, rhs)
    }
}

impl ops::AddAssign<Vector> for Position {
    fn add_assign(&mut self, rhs: Vector) {
        *self = Position::add_vector(*self, rhs);
    }
}

impl ops::Sub<Vector> for Position {
    type Output = Position;

    fn sub(self, rhs: Vector) -> Position {
        Position::add_vector(self, -rhs)
    }
}
