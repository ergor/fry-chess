
use chess_structs::*;
use chess_structs::Piece::*;
use chess_structs::Color::*;


#[derive(Itrator)]
struct KingItr {
    curr: Board,
    next: Board,
    pos: Index2D,
    nr:i32

}
 impl Iterator for KingItr {
     fn new(board: Board, pos: Index2D) -> KingItr {
        KingItr {
            curr:board,
            next:None,
            pos,
            nr:0
        }
     }

     fn next(){
        match  nr{
            1 => {
                let newPos = Index2D{
                    x: pos.x + 1,
                    y: pos.y
                };
                nr += 1;
                doMove(curr, pos, newPos);
            }
            2 => board,
        }
     }

 }


pub fn doMove(board: Board, from: Index2d, to: Index2D) -> Board {
    let mut board = board;
    board[to.y][to.x] = [from.y][from.x];
    board[from.y][from.x] = Piece.Empty;
    let board = board;
    board
}