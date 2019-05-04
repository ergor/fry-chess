
use super::*;

pub fn generate_starting_board() -> Board {
    let mut starter_board = Board::new();

    // white pieces
    for x in 3..5 {
        starter_board.insert(
            piece_defs::common::from_def(
                piece_defs::pawn::def(),
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
    for x in 3..4 {
        starter_board.insert(
            piece_defs::common::from_def(
                piece_defs::pawn::def(),
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

