
use super::{Vector, Position};
use super::board::SIDE_LEN;

pub fn param_ln(t: i32, origin: Position, direction: Vector) -> Position {
    Position {
        x: (direction.x * t + origin.x as i32) as usize,
        y: (direction.y * t + origin.y as i32) as usize,
    }
}

/**
 * Finds the boundary positions on the given an origin and a direction.
 * 
 * x = dx * t + rx:
 *  dx * t + rx = c
 *  dx * t = c -rx
 *  t = (c - rx) / dx
 */
pub fn param_bounds(origin: Position, direction: Vector) -> (i32, i32) {

    fn t(c: i32, r_a: usize, delta_a: i32) -> i32 {
        (c - r_a as i32) / delta_a
    }

    // handle divide by zero cases:
    if let Vector {x, y: 0} = direction {
        return (-x, 8-x);
    } else if let Vector {x: 0, y} = direction {
        return (-y, 8-y);
    }

    let t_x_min = t(0, origin.x, direction.x);
    let t_y_min = t(0, origin.y, direction.y);
    let t_x_max = t(8, origin.x, direction.x);
    let t_y_max = t(8, origin.y, direction.y);

    let t_lower = i32::max(t_x_min, t_y_min);
    let t_upper = i32::min(t_x_max, t_y_max);

    (t_lower, t_upper)
}

pub fn range_pos(origin: Position, direction: Vector) -> (usize, [Position; SIDE_LEN]) {
    // use case: generating paths a piece can move, and make a slice of it
    // for the moves that don't phase through other pieces.
    // use case: generating paths where the king can be attacked, and check
    // if there are any pieces in those squares that can travel that path.
    // dependency: param_bounds

    let mut range = [Position{x: 0, y: 0}; SIDE_LEN];
    let (t_from, t_to) = param_bounds(origin, direction);
    for (i, t) in (t_from..t_to).into_iter().enumerate() {
        range[i] = param_ln(t, origin, direction);
    }
    ((t_to-t_from) as usize, range)
}