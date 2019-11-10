

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black
}

#[derive(Copy, Clone, PartialEq)]
pub struct Index2D {
    pub x: usize,
    pub y: usize,
}

#[derive(Copy, Clone)]
pub struct Board {
    pub squares: [[Option<Piece>; 8]; 8],
    pub turn: Color,
    pub en_passant: Option<Index2D>,
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl Board {
    pub fn get_next_turn(&mut self) -> Color {
        if self.color == Color::White {
            Color::Black
        }else {
            Color::White
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Kind {
    Pawn,
    Bishop,
    Knight,
    Rook,
    King,
    Queen,
    Empty,
}

#[derive(Copy, Clone)]
pub struct Piece{
    pub kind: Kind,
    pub color: Color
}
