mod chess_structs;

use san_rs::*;
use chess_structs::*;
use chess_structs::Piece::*;
use chess_structs::Color::*;

fn main() {
    let board1 = Board{
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

    println!("Hello, world!");

}
