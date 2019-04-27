
pub mod pawn;
pub mod rook;

use super::*;

pub struct PieceDef {
    symbol: char,
    value: i32,
    vector_iterator: fn(&BoardGenerator) -> Option<Vector>
}

pub fn from_def<'a>(def: PieceDef, board: &'a Board<'a>, 
                    color: Color, position: Position) -> Piece<'a> {
    Piece::new(color, def.symbol, def.value,
                board, position, def.vector_iterator)
}

/*
 * Returns the landing square if the position is reachable,
 * or None if unreachable or movement vector i null vector.
 * 
 * # Arguments
 * * `origin` - the current position of the piece
 * * `vect` - the vector to move the piece along
 * * `condition` - closure that must return whether the landing square is legal.
 */
/*
fn apply_vector(origin: Position, vect: Vector2D, condition: &Fn(Position) -> bool) -> Option<Position> {
    if let (0, 0) = vect {
        return None;
    }

    let (dx, dy) = vect;
    let x = (origin % 8) as i32 + dx;
    let y = (origin / 8) as i32 + dy;
    let landing_sq = Board::position((x , y));

    if Board::within_bounds(landing_sq) && condition(landing_sq) {
        Some(landing_sq)
    } else {
        None
    }
}
*/
/*
 * Generate boards by repeatedly adding a movement vector to the position
 * of the piece we're moving, until it cannot be legally moved anymore.
 * Generation starts from the piece's origin position.
 * Can generate for any number of movement vectors.
 * 
 * # Arguments
 * * `origin` - the starting position of the piece
 * * `basis_board` - the board on which the piece stands
 * * `vects` - the movement vectors
 */
/*pub fn gen_iter(origin: Position, basis_board: &Board, vects: Vec<(i32, i32)>) -> Vec<Board> {

    let moving_piece: &Piece = basis_board.piece_at(origin)
        .unwrap();

    let condition = |landing_sq: Position| { 
        let opt_piece = basis_board.piece_at(landing_sq);
        match opt_piece {
            None => true,
            Some(piece) => piece.is_enemy(moving_piece.color)
        }
    };

    let mut generated_boards = Vec::new();

    for vect in vects {
        if let (0, 0) = vect {
            continue;
        }

        let mut opt_landing_sq = apply_vector(origin, vect, &condition);

        loop {
            match opt_landing_sq {
                None => break,
                Some(landing_sq) => {
                    println!("{:?}", landing_sq);

                    let moving_piece: Piece = moving_piece.clone();

                    let mut pieces: HashMap<Position, Piece> = HashMap::new();
                    for (pos, piece) in &basis_board.pieces {
                        if !pos.cmp(origin) { // dont clone the moving piece yet
                            pieces.insert(*pos, piece.clone());
                        }
                    }

                    pieces.insert(landing_sq, moving_piece);

                    generated_boards.push(Board::new(pieces));

                    // advance the loop
                    opt_landing_sq = apply_vector(landing_sq, vect, &condition);
                }
            }
        }
    }

    generated_boards
}*/

/*
pub fn gen_iter<'a>(origin: Position, directions: Vec<Vector2D>, 
                    condition: &'a Fn(Position) -> bool)
                    -> Vec<((i32, i32), &'a Fn(Position) -> bool)> {

    let mut moves = Vec::new();
    let origin = as_vect(origin);

    for direction in directions {
        if let (0, 0) = direction {
            continue;
        }
        //let mut opt_landing_sq = apply_vector(origin, direction, condition);
        let mut vect = direction;
        while Board::within_bounds(Board::position(add_vectors(vect, origin))) {
            println!("gen_iter: {:?} -> {:?}", vect, add_vectors(vect, origin));
            moves.push((vect, condition));
            vect = add_vectors(vect, direction);
        }
    }
    moves
}
*/
// idea:
// one function which just applies every vector as is. this is the fixed one (knights, pawns, etc)
// one function which generates legal, fixed, movement vectors iteratively from direction vectors.
// then calls the fixed vector function to apply them all. (rooks, bishops, etc)
/*
pub fn gen_fixed(origin: Position, basis_board: &Board, 
                 moves: Vec<((i32, i32), &Fn(Position) -> bool)>
                ) -> Vec<Board> {
    let mut generated_boards = Vec::new();

    for (vect, condition) in moves {
        if let (0, 0) = vect {
            println!("{:?}", vect);
            continue;
        }

        println!("{:?}", vect);
        let opt_landing_sq = apply_vector(origin, vect, condition);

        match opt_landing_sq {
            None => continue,
            Some(landing_sq) => {
                println!("{:?}", as_vect(landing_sq));

                let moving_piece: Piece = basis_board.piece_at(origin)
                    .unwrap()
                    .clone();

                //let mut pieces: HashMap<Position, Piece> = HashMap::new();
                let mut board = Board::new();
                for (pos, piece) in basis_board {
                    if pos != origin { // dont clone the moving piece yet
                        board.insert(pos, piece.clone());
                    }
                }

                board.insert(landing_sq, moving_piece);

                generated_boards.push(board);
            }
        }
    }

    generated_boards
}
*/