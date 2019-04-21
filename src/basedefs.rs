
use std::vec::Vec;

pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    fn cmp(&self, pos2: &Position) -> bool {
        self.x == pos2.x && self.y == pos2.y
    }
}

pub enum Color {
    WHITE,
    BLACK
}

pub enum PieceClass {
    PAWN,
    ROOK,
    KNIGHT,
    BISHOP,
    KING,
    QUEEN
}

impl PieceClass {
    fn enum_map(class: &PieceClass) -> (char, i32) {
        match class {
            PieceClass::PAWN =>   ('p',  100),
            PieceClass::ROOK =>   ('r',  500),
            PieceClass::KNIGHT => ('n',  300),
            PieceClass::BISHOP => ('b',  300),
            PieceClass::KING =>   ('k', 9999),
            PieceClass::QUEEN =>  ('q',  900),
        }
    }
}

pub struct Piece {
    color: Color,               // white or black
    piece_class: PieceClass,    // pawn, rook, etc
    position: Position,         // (0,0) top left; (7,7) bottom right
    symbol: char,               // text representation of the piece
    value: i32,                 // the relative value of the piece

    // generates all the possible board states as a result
    // of performing legal moves with this piece  
    generator: fn(&Piece, &Board) -> Vec<Board>
}

impl Piece {
    pub fn character(&self) -> char {
        match self.color {
            Color::BLACK => self.symbol.to_ascii_lowercase(),
            Color::WHITE => self.symbol.to_ascii_uppercase(),
        }
    }

    /**
     * Creates a new piece, and assigns color based on which
     * half the piece is created in.
     */
    pub fn new(color: Color, piece_class: PieceClass, position: Position) -> Piece {
        let (symbol, value) = PieceClass::enum_map(&piece_class);
        let value = match &color {
            Color::WHITE => value,
            Color::BLACK => -value,
        };

        Piece {
            color,
            piece_class,
            position,
            symbol,
            value,
            generator: |_, _| Vec::new()
        }
    }
}

pub struct Board {
    sum: i32,               // board evaluation
    checks: (u32, u32),     // checks against white,black king
    pub pieces: Vec<Piece>, // the pieces on this board
}

impl Board {
    pub fn new() -> Board {
        Board {
            sum: 0,
            checks: (0, 0),
            pieces: Vec::new()
        }
    }

    pub fn piece_at(&self, pos: &Position) -> Option<&Piece> {
        self.pieces.iter()
            .find(|&p| p.position.cmp(pos))
    }

    pub fn generate_starting_board() -> Board {
        let mut starter_board = Board::new();

        let mut push = |piece_class: PieceClass, position: Position| {
            let color = if position.y < 5 {Color::BLACK} else {Color::WHITE};
            starter_board.pieces.push(
                Piece::new(color, piece_class, position));
        };

        // white pieces
        for x in 0..8 {
            push(PieceClass::PAWN, Position::new(x, 6));
        }

        push(PieceClass::ROOK,   Position::new(0, 7));
        push(PieceClass::KNIGHT, Position::new(1, 7));
        push(PieceClass::BISHOP, Position::new(2, 7));
        push(PieceClass::QUEEN,  Position::new(3, 7));
        push(PieceClass::KING,   Position::new(4, 7));
        push(PieceClass::BISHOP, Position::new(5, 7));
        push(PieceClass::KNIGHT, Position::new(6, 7));
        push(PieceClass::ROOK,   Position::new(7, 7));

        // black pieces
        for x in 0..8 {
            push(PieceClass::PAWN, Position::new(x, 1));
        }

        push(PieceClass::ROOK,   Position::new(0, 0));
        push(PieceClass::KNIGHT, Position::new(1, 0));
        push(PieceClass::BISHOP, Position::new(2, 0));
        push(PieceClass::QUEEN,  Position::new(3, 0));
        push(PieceClass::KING,   Position::new(4, 0));
        push(PieceClass::BISHOP, Position::new(5, 0));
        push(PieceClass::KNIGHT, Position::new(6, 0));
        push(PieceClass::ROOK,   Position::new(7, 0));

        starter_board
    }
}

fn gen_pawn(piece: Piece, board: Board) -> Vec<Board> {
    Vec::new()
}

fn dummy_gen(piece: Piece, board: Board) -> Vec<Board> {
    Vec::new()
}
