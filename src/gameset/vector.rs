
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
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

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}
