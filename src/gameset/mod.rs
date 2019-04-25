
mod piece_defs;

use std::vec::Vec;

pub const BOARD_SZ: usize = 64;
pub type Squares = [Option<Piece>; BOARD_SZ];
pub type Vector2D = (i32, i32);
pub type Position = usize;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Color {
    WHITE,
    BLACK
}

#[derive(Copy, Clone)]
pub struct Piece {
    color: Color,               // white or black
    symbol: char,               // text representation of the piece
    value: i32,                 // the relative value of the piece
    pub generator: fn(Position, &Board) -> Vec<Board>
}

impl Piece {
    pub fn new(color: Color, symbol: char, value: i32,
               generator: fn(Position, &Board) -> Vec<Board>) -> Piece {
        let value = match color {
            Color::WHITE => value,
            Color::BLACK => -value,
        };

        Piece { color, symbol, value, generator }
    }

    pub fn character(&self) -> char {
        match self.color {
            Color::BLACK => self.symbol.to_ascii_lowercase(),
            Color::WHITE => self.symbol.to_ascii_uppercase(),
        }
    }

    pub fn is_enemy(&self, my_color: Color) -> bool {
        self.color != my_color
    }
}

pub struct Board {
    sum: i32,               // board evaluation
    checks: (u32, u32),     // checks against white,black king
    pub squares: Squares    // (0,0) top left; (7,7) bottom right
}

impl Board {
    pub fn new() -> Board {
        Board {
            sum: 0,
            checks: (0, 0),
            squares: [None; BOARD_SZ]
        }
    }

    pub fn position(pos: Vector2D) -> Position {
        let (x, y) = pos;
        if x < 0 || y < 0 {
            panic!("components less than 0");
        }
        ((y << 3) + x) as Position
    }

    pub fn within_bounds(pos: Position) -> bool {
        pos < BOARD_SZ
    }

    pub fn insert(&mut self, pos: Position, piece: Piece) {
        self.squares[pos] = Some(piece);
    }

    pub fn piece_at(&self, pos: Position) -> Option<&Piece> {
        match &self.squares[pos] {
            Some(p) => Some(p),
            None => None
        }
    }
}

pub struct BoardIterator<'a> {
    squares: &'a Squares,
    index: Position
}

impl<'a> IntoIterator for &'a Board {
    type Item = (Position, &'a Piece);
    type IntoIter = BoardIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BoardIterator {
            squares: &self.squares,
            index: 0
        }
    }
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = (Position, &'a Piece);

    // FIXME: infinite loop ahead
    fn next(&mut self) -> Option<(Position, &'a Piece)> {
        loop {
            let i = self.index;
            match &self.squares[i] {
                None => {
                    if i < BOARD_SZ {
                        self.index += 1;
                    } else {
                        return None;
                    }
                },
                Some(piece) => return Some((i, piece))
            }
        }
    }
}


pub fn generate_starting_board() -> Board {
    let mut starter_board = Board::new();

    // white pieces
    for x in 1..8 {
        starter_board.insert(
            Board::position((x, 6)),
            piece_defs::new(piece_defs::pawn::def(), Color::WHITE));
        //starter_board.pieces.push(Box::new(Pawn::new(Color::WHITE, Position::new(x, 6))));
    }

    starter_board.insert(Board::position((0, 7)), piece_defs::new(piece_defs::rook::def(), Color::WHITE));
    //starter_board.pieces.push(Box::new(Rook::new(Color::WHITE, Position::new(0, 7))));
    //push(PieceClass::KNIGHT, Position::new(1, 7));
    //push(PieceClass::BISHOP, Position::new(2, 7));
    //push(PieceClass::QUEEN,  Position::new(3, 7));
    //push(PieceClass::KING,   Position::new(4, 7));
    //push(PieceClass::BISHOP, Position::new(5, 7));
    //push(PieceClass::KNIGHT, Position::new(6, 7));
    //push(PieceClass::ROOK,   Position::new(7, 7));

    // black pieces
    for x in 0..8 {
        starter_board.insert(
            Board::position((x, 1)),
            piece_defs::new(piece_defs::pawn::def(), Color::BLACK));
    }

    //push(PieceClass::ROOK,   Position::new(0, 0));
    //push(PieceClass::KNIGHT, Position::new(1, 0));
    //push(PieceClass::BISHOP, Position::new(2, 0));
    //push(PieceClass::QUEEN,  Position::new(3, 0));
    //push(PieceClass::KING,   Position::new(4, 0));
    //push(PieceClass::BISHOP, Position::new(5, 0));
    //push(PieceClass::KNIGHT, Position::new(6, 0));
    //push(PieceClass::ROOK,   Position::new(7, 0));

    starter_board
}

