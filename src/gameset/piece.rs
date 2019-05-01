
use super::{Position, Vector};
use super::board::{Board, Squares, BoardGenerator, BoardGeneratorState, BOARD_SZ};

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
