pub mod defaults;
pub mod piece_defs;

use super::math::*;

pub const BOARD_SZ: usize = 64;
pub const SIDE_LEN: usize = 8;
pub type Squares = [[Option<Piece>; SIDE_LEN]; SIDE_LEN]; // [y][x]?


pub struct BoardMeta {
    checks: i32,                    // checks against oneself's king
    left_rook_moved: bool,          // whether left rooks has been moved
    right_rook_moved: bool,         // whether right rook has been moved
    king_moved: bool,               // whether the king has been moved
    en_passant: [bool; SIDE_LEN]    // pawns that can be lost via en passant
}

impl BoardMeta {
    pub fn new() -> BoardMeta {
        BoardMeta {
            checks: 0,
            left_rook_moved: false,
            right_rook_moved: false,
            king_moved: false,
            en_passant: [false; SIDE_LEN]
        }
    }
}

pub struct Board {
    sum: i32,               // board evaluation
    meta_white: BoardMeta,  // metadata for white player
    meta_black: BoardMeta,  // metadata for black player
    player: Color,          // which player has the turn
    pub squares: Squares    // (0,0) top left; (7,7) bottom right
}

impl Board{
    pub fn new() -> Board {
        Board {
            sum: 0,
            meta_white: BoardMeta::new(),
            meta_black: BoardMeta::new(),
            player: Color::WHITE,
            squares: [[None; SIDE_LEN]; SIDE_LEN],
        }
    }

    pub fn within_bounds(pos: Position) -> bool {
        pos.x < SIDE_LEN && pos.y < SIDE_LEN
    }

    pub fn insert_mut(&mut self, piece: Piece) {
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
    pub fn evaluate_mut(&mut self, previous_board: &Board) -> bool {
        let mut sum = 0;

        for piece in self.pieces() {
            sum += piece.value;
        }

        let king_exposed = match self.player {
            Color::WHITE => (self.meta_white.checks - previous_board.meta_white.checks) > 0,
            Color::BLACK => (self.meta_black.checks - previous_board.meta_black.checks) > 0
        };
        self.sum = sum;
        return !king_exposed;
    }

    pub fn generate(&self, moving_piece: &Piece) -> Vec<Board> {
        let mut boards = Vec::new();

        for position in(moving_piece.generator)(self, moving_piece) {
            // copy all pieces over, except for the moving piece
            let mut new_board = Board::new();
            for piece in self.pieces() {
                if piece.is(moving_piece) {
                    continue;
                }
                let piece = *piece;
                new_board.insert_mut(piece);
            }
            // make copy of moving piece and update position
            let mut moving_piece = *moving_piece;
            moving_piece.position = position;

            new_board.insert_mut(moving_piece);
            boards.push(new_board);
        }

        boards
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
    pub generator: fn(&Board, &Piece) -> Vec<Position>
}

impl Piece {
    pub fn new(color: Color, 
               symbol: char, 
               value: i32, 
               position: Position,
               generator: fn(&Board, &Piece) -> Vec<Position>)
               -> Piece {
        let value = match color {
            Color::WHITE => value,
            Color::BLACK => -value,
        };

        Piece { color, symbol, value, position, generator }
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
