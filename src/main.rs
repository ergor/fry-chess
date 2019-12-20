use crate::chess_structs::{Board, Piece, Color};
use crate::chess_structs::Kind::{Pawn, King};
//use create::generator::MoveItr;

mod chess_structs;
mod generator;

//use san_rs::*;
//use chess_structs::*;
//use chess_structs::Color::*;

fn main() {
    let _board: Board = Board {
        squares: [
            [None; 8],
            [None; 8],
            [None; 8],
            [None; 8],
            [None, None, None, None, Some(Piece{kind:Pawn, color:Color::Black}), None, None, None],
            [None, None, None, None, Some(Piece{kind:King, color:Color::White}), None, None, None],
            [None; 8],
            [None; 8],
        ],
        turn: Color::White,
        en_passant: None,
        white_kingside: false,
        white_queenside: false,
        black_kingside: false,
        black_queenside: false
    };
//    let move_itr = MoveItr::new(board);
//    let board1 = Board{
//        squares : [
//            [Rook(Black),Knight(Black),Bishop(Black),Queen(Black),King(Black),Bishop(Black),Knight(Black),Rook(Black)],
//            [Pawn(Black); 8],
//            [Empty; 8],
//            [Empty; 8],
//            [Empty; 8],
//            [Empty; 8],
//            [Pawn(White); 8],
//            [Rook(White),Knight(White),Bishop(White),Queen(White),King(White),Bishop(White),Knight(White),Rook(White)],
//        ],
//        turn: Color::White,
//        en_passant: None,
//        white_kingside: false,
//        white_queenside: false,
//        black_kingside: false,
//        black_queenside: false,
//    };

    println!("Hello, world!");

}
