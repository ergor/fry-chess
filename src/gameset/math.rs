
use super::{Vector, Position};
use super::board::SIDE_LEN;

/**
 * x = (dx * t) + rx
 * y = (dy * t) + ry
 * 
 * (dx * t) + rx = x
 * dx * t = x - rx
 * t = (x - rx) / dx
 * 
 * y = (dy * t) + ry
 * y = (dy * (x - rx) / dx) + ry
 * y = (dy/dx (x - rx)) + ry
 */
pub fn y(x: i32, origin: Position, direction: Vector) -> usize {
    let result = ((direction.y/direction.x) * (x - origin.x as i32)) + origin.y as i32;
    i32::min(7, i32::max(0, result)) as usize
}

pub fn x(y: i32, origin: Position, direction: Vector) -> usize {
    let result = ((direction.x/direction.y) * (y - origin.y as i32)) + origin.x as i32;
    i32::min(7, i32::max(0, result)) as usize
}


pub fn range_pos(origin: Position, direction: Vector) -> [Position; SIDE_LEN] {
    // use case: generating paths a piece can move, and make a slice of it
    // for the moves that don't phase through other pieces.
    // use case: generating paths where the king can be attacked, and check
    // if there are any pieces in those squares that can travel that path.
    // dependency: param_bounds

    let mut line = [Position{x: 0, y: 0}; SIDE_LEN];
    let x_start = x(0, origin, direction);
    let y_start = y(0, origin, direction);
    let x_end = x(7, origin, direction);
    let y_end = x(7, origin, direction);

    for (i, (x, y)) in (x_start..x_end).into_iter().zip(y_start..y_end).enumerate() {
        line[i] = Position::new(x, y);
    }

    line
}