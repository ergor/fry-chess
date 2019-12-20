mod chess_structs;

use san_rs;
use chess_structs::*;
use chess_structs::Piece::*;
use chess_structs::Color::*;

fn main() {
    let board1 = Board {
        squares : [
            [Rook(Black),Knight(Black),Bishop(Black),Queen(Black),King(Black),Bishop(Black),Knight(Black),Rook(Black)],
            [Pawn(Black); 8],
            [Empty; 8],
            [Empty; 8],
            [Empty; 8],
            [Empty; 8],
            [Pawn(White); 8],
            [Rook(White),Knight(White),Bishop(White),Queen(White),King(White),Bishop(White),Knight(White),Rook(White)],
        ],
        turn: Color::White,
        en_passant: None,
        white_kingside: false,
        white_queenside: false,
        black_kingside: false,
        black_queenside: false,
    };

    board1.print();
}


impl Board {
    fn print(&self) {
        println!("hello");

        for rank in 0..8 {
            print!(" {} | ", 8- (rank));
            for file in 0..8 {
                print!(" {} ", self.squares[rank][file].to_char());
            }
            println!();
        }

        println!("   +  -  -  -  -  -  -  -  -\n");
        print!("     ");
        for c in "abcdefgh".chars() {
            print!(" {} ", c);
        }
        println!();
    }
}

impl Piece {
    fn to_char(&self) -> char {
        match self {
            Piece::Pawn(color) => {
                match color {
                    White => 'P',
                    Black => 'p',
                }
            }
            Piece::Bishop(color) => {
                match color {
                    White => 'B',
                    Black => 'b',
                }
            }
            Piece::King(color) => {
                match color {
                    White => 'K',
                    Black => 'k',
                }
            }
            Piece::Knight(color) => {
                match color {
                    White => 'N',
                    Black => 'n',
                }
            }
            Piece::Queen(color) => {
                match color {
                    White => 'Q',
                    Black => 'q',
                }
            }
            Piece::Rook(color) => {
                match color {
                    White => 'R',
                    Black => 'r',
                }
            }
            Piece::Empty => {
                '.'
            }
        }
    }
}