
pub mod piece_defs;

use std::vec::Vec;

pub const BOARD_SZ: usize = 64;
pub const SIDE_LEN: usize = 8;
pub type Squares<'a> = [[Option<Piece<'a>>; SIDE_LEN]; SIDE_LEN]; // [y][x]?

#[derive(Copy, Clone)]
pub struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    pub fn add(&self, vect: Vector) -> Vector {
        Vector {
            x: self.x + vect.x,
            y: self.y + vect.y,
        }
    }

    pub fn add_position(&self, position: Position) -> Vector {
        Vector {
            x: self.x + position.x as i32,
            y: self.y + position.y as i32,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position {x, y}
    }

    pub fn add_vector(&self, vector: Vector) -> Position {
        Position {
            x: self.x + vector.x as usize,
            y: self.y + vector.y as usize,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Color {
    WHITE,
    BLACK
}

#[derive(Copy, Clone)]
pub struct Piece<'a> {
    color: Color,               // white or black
    symbol: char,               // text representation of the piece
    value: i32,                 // the relative value of the piece
    board: &'a Board<'a>,       // the board this piece is on
    pub position: Position,         // its position on the board
    vector_iterator: fn(&BoardGenerator) -> Option<Vector>
}

impl<'a> Piece<'a> {
    pub fn new(color: Color, symbol: char, value: i32,
               board: &'a Board, position: Position,
               vector_iterator: fn(&BoardGenerator) -> Option<Vector>)
               -> Piece<'a> {
        let value = match color {
            Color::WHITE => value,
            Color::BLACK => -value,
        };

        Piece { color, symbol, value, board, position, vector_iterator }
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

    pub fn apply_move(&self, vector: Vector) {
        self.position = self.position.add_vector(vector);
    }
}

enum GeneratorState {
    // get next legal vector directly from piece's directions LUT.
    // post increment index.
    Next(usize),

    // get next legal vector by adding vector from piece's directions LUT to the
    // accumulated value. don't increment index.
    Continue(usize, Vector),
}

struct BoardGenerator<'a> {
    piece: &'a Piece<'a>,
    state: GeneratorState,
}

impl<'a> IntoIterator for &'a Piece<'a> {
    type Item = Board<'a>;
    type IntoIter = BoardGenerator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BoardGenerator {
            piece: &self,
            state: GeneratorState::Next(0),
        }
    }
}

impl<'a> Iterator for BoardGenerator<'a> {
    type Item = Board<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let basis_board = self.piece.board;
        let delta_vect = (self.piece.vector_iterator)(&self);
        // if next_index is Some index, check if condition is met
        // to legally generate board. if illegal, try again as long
        // as next_index is not None
        match delta_vect {
            None => None,
            Some(vect) => {
                let board = Board::new();
                for piece in basis_board {
                    if piece as *const _ == self.piece as *const _ {
                        continue;
                    }
                    board.insert(piece.clone());
                }
                let piece = self.piece.clone();
                piece.position.add_vector(vect);

                board.insert(piece);
                Some(board)
            }
        }
    }
}

pub struct Board<'a> {
    sum: i32,                   // board evaluation
    checks: (u32, u32),         // checks against white,black king
    pub squares: Squares<'a>    // (0,0) top left; (7,7) bottom right
}

impl<'a> Board<'a> {
    pub fn new() -> Board<'a> {
        Board {
            sum: 0,
            checks: (0, 0),
            squares: [[None; SIDE_LEN]; SIDE_LEN],
        }
    }

    pub fn within_bounds(pos: Position) -> bool {
        pos.x >= 0 && pos.x < SIDE_LEN && pos.y >= 0 && pos.y < SIDE_LEN
    }

    pub fn insert(&'a mut self, piece: Piece<'a>) {
        let pos = piece.position;
        self.squares[pos.y][pos.x] = Some(piece);
    }

    pub fn piece_at(&self, pos: Position) -> Option<&Piece> {
        self.squares[pos.y][pos.x].as_ref()
    }
}

pub struct PieceIterator<'a> {
    squares: &'a Squares<'a>,
    pos: Position
}

impl<'a> IntoIterator for &'a Board<'a> {
    type Item = &'a Piece<'a>;
    type IntoIter = PieceIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PieceIterator {
            squares: &self.squares,
            pos: Position {x: 0, y: 0},
        }
    }
}

impl<'a> Iterator for PieceIterator<'a> {
    type Item = &'a Piece<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        for y in self.pos.y .. SIDE_LEN {
            for x in self.pos.x .. SIDE_LEN {
                match &self.squares[y][x] {
                    None => continue,
                    Some(piece) => {
                        self.pos = Position {x, y};
                        return Some(piece);
                    }
                }
            }
        }
        None
    }
}


pub fn generate_starting_board<'a>() -> Board<'a> {
    let mut starter_board = Board::new();

    // white pieces
    for x in 0..8 {
        starter_board.insert(
            piece_defs::from_def(
                piece_defs::pawn::def(),
                &starter_board,
                Color::WHITE,
                Position::new(x, 6)));
        //starter_board.pieces.push(Box::new(Pawn::new(Color::WHITE, Position::new(x, 6))));
    }

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
            piece_defs::from_def(
                piece_defs::pawn::def(),
                &starter_board,
                Color::BLACK,
                Position::new(x, 1)));
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

