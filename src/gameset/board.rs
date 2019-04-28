
use super::{Position, Vector};
use super::piece::{Piece, PieceIterator};

pub const BOARD_SZ: usize = 64;
pub const SIDE_LEN: usize = 8;
pub type Squares = [[Option<Piece>; SIDE_LEN]; SIDE_LEN]; // [y][x]?

pub struct Board {
    sum: i32,                   // board evaluation
    checks: (u32, u32),         // checks against white,black king
    pub squares: Squares    // (0,0) top left; (7,7) bottom right
}

impl Board{
    pub fn new() -> Board {
        Board {
            sum: 0,
            checks: (0, 0),
            squares: [[None; SIDE_LEN]; SIDE_LEN],
        }
    }

    pub fn within_bounds(pos: Position) -> bool {
        pos.x < SIDE_LEN && pos.y < SIDE_LEN
    }

    pub fn insert(&mut self, piece: Piece) {
        let pos = piece.position;
        self.squares[pos.y][pos.x] = Some(piece);
    }

    pub fn piece_at(&self, pos: Position) -> Option<&Piece> {
        self.squares[pos.y][pos.x].as_ref()
    }

    pub fn pieces<'a>(&'a self) -> PieceIterator<'a> {
        PieceIterator {
            squares: &self.squares,
            index: 0,
        }
    }
}

/**
 * 
 */
pub enum BoardGeneratorState {
    // get next legal vector from piece's directions LUT.
    // post increment LUT index.
    Next(usize),

    // get next legal vector by adding vector from piece's directions LUT to the
    // accumulated value. don't increment LUT index.
    Accumulate(usize, Vector),
}

pub struct BoardGenerator<'a> {
    pub piece: &'a Piece,
    pub basis_board: &'a Board,
    pub state: BoardGeneratorState,
}

/**
 * 
 */
impl<'a> Iterator for BoardGenerator<'a> {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        match self.piece.next_vector(self) {
            None => None,
            Some(vect) => {
                let mut board = Board::new();
                for piece in self.basis_board.pieces() {
                    if piece.is(self.piece) {
                        //println!("\tskipped");
                        continue;
                    }
                    board.insert(piece.clone());
                    //println!("\tcopied");
                }
                let mut piece = self.piece.clone();
                piece.position += vect;

                board.insert(piece);
                Some(board)
            }
        }
    }
}
