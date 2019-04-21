
use super::super::{Piece, PieceData, Board, Color, Position};

const SYMBOL: char = 'p';
const VALUE: i32 = 100;

pub struct Pawn {
    piece_data: PieceData
}

impl Piece for Pawn {
    fn piece(&self) -> &PieceData {
        &self.piece_data
    }

    fn generate(&self, board: &Board) -> Vec<Board> {
        Vec::new()
    }
}

impl Pawn {
    pub fn new(color: Color,
               position: Position
        ) -> Pawn {
        Pawn {
            piece_data: PieceData::new(
                color,
                SYMBOL,
                VALUE,
                position)
        }
    }
}
