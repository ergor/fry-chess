

#[derive(Copy, Clone)]
pub enum Color {
    White,
    Black
}

#[derive(Copy, Clone)]
pub struct Index2D {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
pub struct Board {
    pub squares: [[Piece; 8]; 8],
    pub turn: Color,
    pub en_passant: Option<Index2D>,
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

#[derive(Copy, Clone)]
pub enum Kind {
    Pawn(Color),
    Bishop(Color),
    Knight(Color),
    Rook(Color),
    King(Color),
    Queen(Color),
    Empty,
}

#[derive(Copy, Clone)]
pub struct Piece{
    kind: Kind,
    color: Color
}
