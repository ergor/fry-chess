
pub mod piece_defs;

use super::math::*;
use san_rs::{Move, MoveKind};

/* BOARD ***********************************************************************
 */

pub const BOARD_SZ: usize = 64;
pub const SIDE_LEN: usize = 8;
pub type Squares = [[Option<Piece>; SIDE_LEN]; SIDE_LEN]; // [y][x]


#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Board {
    pub sum: i32,               // board evaluation
    pub meta_white: BoardMeta,  // metadata for white player
    pub meta_black: BoardMeta,  // metadata for black player
    pub player: Color,          // which player has the turn

    // (0,0) top left; (7,7) bottom right
    pub squares: [[Option<Piece>; SIDE_LEN]; SIDE_LEN]
}

impl Board{
    pub fn new() -> Board {
        Board {
            sum: 0,
            meta_white: BoardMeta::new(),
            meta_black: BoardMeta::new(),
            player: Color::White,
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

    pub fn owned_piece_at(&self, pos: Position) -> Option<Piece> {
        self.squares[pos.y][pos.x]
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
            sum += piece.value();
        }

        let king_exposed = match self.player {
            Color::White => (self.meta_white.checks - previous_board.meta_white.checks) > 0,
            Color::Black => (self.meta_black.checks - previous_board.meta_black.checks) > 0
        };
        self.sum = sum;
        return !king_exposed;
    }

    pub fn generate(&self, moving_piece: &Piece) -> Vec<Board> {
        let mut boards = Vec::new();

        for position in moving_piece.generator()(self, moving_piece) {
            let mut new_board = Board::new();
            // after this move, of course it's the other player's turn
            match self.player {
                Color::White => new_board.player = Color::Black,
                Color::Black => new_board.player = Color::White
            }
            // copy all pieces over, except for the moving piece
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

    pub fn from_state(state: fen::BoardState) -> Board {
        fn convert_piece(piece: (usize, &Option<fen::Piece>)) -> Option<Piece> {
            let (i, p) = piece;
            match p {
                None => None,
                Some(p) => {
                    let col = i % 8; // cols are same as mine.
                    let row = i / 8; // but rows start at bottom.
                    let pos = Position::new(col, 7 - row); // thus: invert rows.
                    let color = match p.color {
                        fen::Color::Black => Color::Black,
                        fen::Color::White => Color::White
                    };
                    let kind = match p.kind {
                       fen::PieceKind::Pawn => PieceKind::Pawn,
                       fen::PieceKind::Knight => PieceKind::Knight,
                       fen::PieceKind::Bishop => PieceKind::Bishop,
                       fen::PieceKind::Rook => PieceKind::Rook,
                       fen::PieceKind::Queen => PieceKind::Queen,
                       fen::PieceKind::King => PieceKind::King
                    };
                    Some(Piece::new(color, pos, kind))
                }
            }
        }

        let mut board = Board::new();
        state.pieces.iter()
            .enumerate()
            .filter_map(|e| convert_piece(e))
            .for_each(|p| board.insert_mut(p));
        return board;
    }

    pub fn try_san_move(&self, mov: Move) -> Option<Board> {
        match &mov.move_kind {
            MoveKind::Normal(src, dst) => {

                fn is_candidate(piece: &Piece, kind: &san_rs::Piece, src: &san_rs::Position) -> bool {
                    piece.kind == PieceKind::from_san_piece(kind)
                        && src.x.map_or(true, |x| piece.position.x == x) 
                        && src.y.map_or(true, |y| piece.position.y == y)
                }

                let landing_sq = Position::new(dst.x.unwrap(), dst.y.unwrap());
                let potential = self.pieces()
                    // all pieces of correct color, kind and original position
                    .filter(|piece| piece.color == self.player && is_candidate(piece, &mov.piece, &src))
                    // generate all their possible moves
                    .flat_map(|piece| self.generate(piece))
                    // and get the ones that land on the correct square
                    .filter(|board| board.piece_at(landing_sq).map_or(false, |piece| piece.kind == PieceKind::from_san_piece(&mov.piece)))
                    .collect::<Vec<_>>();

                if potential.len() == 1 {
                    return Some(potential[0].clone());
                }
                //println!("{:?}", potential);
                println!("found {} move candidates", potential.len());
                None
            },
            MoveKind::Castle(side) => {
                None
            }
        }
    }
}

/* PIECE ***********************************************************************
 */

type FnGenerator = fn(board: &Board, piece: &Piece) -> Vec<Position>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    White,
    Black
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

impl PieceKind {
    pub fn from_san_piece(piece: &san_rs::Piece) -> PieceKind {
        match piece {
            san_rs::Piece::Pawn => PieceKind::Pawn,
            san_rs::Piece::Knight => PieceKind::Knight,
            san_rs::Piece::Bishop => PieceKind::Bishop,
            san_rs::Piece::Rook => PieceKind::Rook,
            san_rs::Piece::Queen => PieceKind::Queen,
            san_rs::Piece::King => PieceKind::King
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    pub color: Color,       // white or black
    pub position: Position, // its position on the board
    pub kind: PieceKind,    // type, value, character, generator
}

impl Piece {
    pub fn new(color: Color, 
               position: Position,
               kind: PieceKind)
               -> Piece {
        Piece { color, position, kind }
    }

    pub fn character(&self) -> char {
        let symbol = match self.kind {
            PieceKind::Pawn => 'p',
            PieceKind::Knight => 'n',
            PieceKind::Bishop => 'b',
            PieceKind::Rook => 'r',
            PieceKind::Queen => 'q',
            PieceKind::King => 'k'
        };
        match self.color {
            Color::Black => symbol.to_ascii_lowercase(),
            Color::White => symbol.to_ascii_uppercase(),
        }
    }

    pub fn value(&self) -> i32 {
        let val = match self.kind {
            PieceKind::Pawn => 100,
            PieceKind::Knight => 300,
            PieceKind::Bishop => 300,
            PieceKind::Rook => 500,
            PieceKind::Queen => 900,
            PieceKind::King => 999999
        };
        match self.color {
            Color::Black => -val,
            Color::White => val,
        }
    }

    pub fn generator(&self) -> FnGenerator {
        match self.kind {
            PieceKind::Pawn => piece_defs::pawn::generator,
            PieceKind::Knight => piece_defs::knight::generator,
            PieceKind::Rook => piece_defs::rook::generator,
            _ => panic!("generator not implemented")
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
