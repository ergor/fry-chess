pub mod defaults;
pub mod piece_defs;

use super::math::*;

pub const BOARD_SZ: usize = 64;
pub const SIDE_LEN: usize = 8;
pub type Squares = [[Option<Piece>; SIDE_LEN]; SIDE_LEN]; // [y][x]?

pub struct Board {
    sum: i32,               // board evaluation
    checks: (u32, u32),     // checks against white,black king
    player: Color,          // which player has the turn
    pub squares: Squares    // (0,0) top left; (7,7) bottom right
}

impl Board{
    pub fn new() -> Board {
        Board {
            sum: 0,
            checks: (0, 0),
            player: Color::WHITE,
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

    /**
     * Evaluates score of board and checks if its a legal state.
     * Returns whether the state is legal.
     * 
     * * sum of values of pieces
     * * bonus points for pieces that defends other pieces
     * * bonus points for pieces attacking enemy pieces
     */
    pub fn evaluate(&mut self, previous_board: &Board) -> bool {
        let mut sum = 0;

        for piece in self.pieces() {
            sum += piece.value;
        }

        let king_exposed = match self.player {
            Color::WHITE => (self.checks.0 - previous_board.checks.0) > 0,
            Color::BLACK => (self.checks.1 - previous_board.checks.1) > 0
        };
        self.sum = sum;
        return !king_exposed;
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
        let moving_piece = self.piece;
        match moving_piece.next_vector(self) {
            None => None,
            Some(vect) => {
                let mut board = Board::new(); // can i init+copy all in one op?
                for piece in self.basis_board.pieces() {
                    if piece.is(moving_piece) { continue; }
                    board.insert(piece.clone());
                }
                let mut piece = moving_piece.clone();
                piece.position += vect;

                board.insert(piece);
                if board.evaluate(self.basis_board) {
                    Some(board)
                } else {
                    self.next()
                }
            }
        }
    }
}

/* *********************************************/

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Color {
    WHITE,
    BLACK
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub color: Color,               // white or black
    symbol: char,               // text representation of the piece
    pub value: i32,                 // the relative value of the piece
    pub position: Position,         // its position on the board
    vector_iterator: fn(&mut BoardGenerator) -> Option<Vector>
}

impl Piece {
    pub fn new(color: Color, symbol: char, value: i32, position: Position,
               vector_iterator: fn(&mut BoardGenerator) -> Option<Vector>)
               -> Piece {
        let value = match color {
            Color::WHITE => value,
            Color::BLACK => -value,
        };

        Piece { color, symbol, value, position, vector_iterator }
    }

    pub fn character(&self) -> char {
        match self.color {
            Color::BLACK => self.symbol.to_ascii_lowercase(),
            Color::WHITE => self.symbol.to_ascii_uppercase(),
        }
    }

    pub fn is_enemy_of(&self, piece: &Piece) -> bool {
        self.color != piece.color
    }

    pub fn next_vector(&self, board_generator: &mut BoardGenerator) -> Option<Vector> {
        (self.vector_iterator)(board_generator)
    }

    pub fn generator<'a>(&'a self, basis_board: &'a Board) -> BoardGenerator<'a> {
        BoardGenerator {
            piece: self,
            basis_board,
            state: BoardGeneratorState::Next(0),
        }
    }

    pub fn is(&self, piece: &Piece) -> bool {
        self as *const _ == piece as *const _
    }
}

pub struct PieceIterator<'a> {
    pub squares: &'a Squares,
    pub index: usize
}

impl<'a> Iterator for PieceIterator<'a> {
    type Item = &'a Piece;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < BOARD_SZ {
            let x = self.index & 7;     // n % 2^i = n & (2^i - 1)
            let y = self.index >> 3;    // n / 2^i = n >> i
            self.index += 1;
            match &self.squares[y][x] {
                None => continue,
                Some(piece) => return Some(piece)
            }
        }
        None
    }
}
