
use super::super::{Piece, PieceData, Board, Color, Position};

const SYMBOL: char = 'r';
const VALUE: i32 = 500;

pub struct Rook {
    piece_data: PieceData
}

impl Piece for Rook {
    fn piece(&self) -> &PieceData {
        &self.piece_data
    }

    fn generate(&self, board: &Board) -> Vec<Board> {
        let mut generated = Vec::new();

        

        generated
    }
}

impl Rook {
    pub fn new(color: Color,
               position: Position
        ) -> Rook {
        Rook {
            piece_data: PieceData::new(
                color,
                SYMBOL,
                VALUE,
                position)
        }
    }
}