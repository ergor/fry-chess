
use super::gameset::position::Position;
use super::gameset::vector::Vector;
use super::gameset::math;

#[test]
fn test_range_pos() {
    let mut grid = [[' '; 8]; 8];
    let origin = Position::new(3, 5);
    let direction = Vector::new(0, 1);

    let (len, range) = math::range_pos(origin, direction);

    for (i, pos) in range.iter().enumerate() {
        println!("{:?}", pos);
        if i == len {
            break;
        }
        grid[pos.y][pos.x] = '*';
    }

    grid[origin.y][origin.x] = 'X';

    let mut stdout = super::StandardStream::stdout(super::ColorChoice::Auto);
    for y in 0..8 {
        for x in 0..8 {
            super::print_sq(
                &mut stdout,
                grid[y][x],
                if (x+y) & 1 == 0 {super::WHITE_SQ} else {super::BLACK_SQ}
            ).unwrap();
        }
        println!("");
    }
}