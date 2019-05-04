
use std::ops;



/* ***********************************************/

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Vector {
        Vector { x, y }
    }

    pub fn add(&self, vect: Vector) -> Vector {
        Vector {
            x: self.x + vect.x,
            y: self.y + vect.y,
        }
    }

    //pub fn add_position(vector: Vector, position: Position) -> Vector {
    //    Vector {
    //        x: vector.x + position.x as i32,
    //        y: vector.y + position.y as i32,
    //    }
    //}
}

impl ops::Mul<i32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: i32) -> Vector {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}


/* ***********************************************/

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