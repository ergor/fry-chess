
pub mod piece_defs;

use std::ops;

pub const BOARD_SZ: usize = 64;
pub const SIDE_LEN: usize = 8;
pub type Squares = [[Option<Piece>; SIDE_LEN]; SIDE_LEN]; // [y][x]?

#[derive(Copy, Clone, Debug)]
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

    //pub fn add_position(vector: Vector, position: Position) -> Vector {
    //    Vector {
    //        x: vector.x + position.x as i32,
    //        y: vector.y + position.y as i32,
    //    }
    //}
}

impl ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position {x, y}
    }

    pub fn add_vector(position: Position, vector: Vector) -> Position {
        let x = position.x as i32 + vector.x;
        let y = position.y as i32 + vector.y;
        Position {
            x: x as usize,
            y: y as usize,
        }
    }
}

impl ops::Add<Vector> for Position {
    type Output = Position;

    fn add(self, rhs: Vector) -> Position {
        Position::add_vector(self, rhs)
    }
}

impl ops::AddAssign<Vector> for Position {
    fn add_assign(&mut self, rhs: Vector) {
        *self = Position::add_vector(*self, rhs);
    }
}

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

    pub fn is_enemy(&self, my_color: Color) -> bool {
        self.color != my_color
    }

    // FIXME: this must be done at board level
    pub fn apply_move(&mut self, vector: Vector) {
        //self.position = self.position.add_vector(vector);
    }

    pub fn generator<'a>(&'a self, basis_board: &'a Board) -> BoardGenerator<'a> {
        BoardGenerator {
            piece: self,
            basis_board,
            state: GeneratorState::Next(0),
        }
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

pub struct BoardGenerator<'a> {
    piece: &'a Piece,
    basis_board: &'a Board,
    state: GeneratorState,
}

impl<'a> Iterator for BoardGenerator<'a> {
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.piece.vector_iterator)(self) {
            None => None,
            Some(vect) => {
                let mut board = Board::new();
                for piece in self.basis_board.pieces() {
                    if piece as *const _ == self.piece as *const _ {
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

pub struct PieceIterator<'a> {
    squares: &'a Squares,
    index: usize
}

impl<'a> Iterator for PieceIterator<'a> {
    type Item = &'a Piece;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < BOARD_SZ {
            let x = self.index & 7;     // n % 2^i = n & (2^i - 1)
            let y = self.index >> 3;    // n / 2^i = n >> i
            self.index += 1;
            //println!("probing: x {}, y {}", x, y);
            match &self.squares[y][x] {
                None => continue,
                Some(piece) => {
                    //println!("\tfound");
                    return Some(piece);
                }
            }
        }
        None
    }
}


pub fn generate_starting_board() -> Board {
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

