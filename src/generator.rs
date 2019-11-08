
use chess_structs::*;
use chess_structs::Piece::*;
use chess_structs::Color::*;

// idea: generate most likely board first, specific for black and white

#[derive(Itrator)]
struct MoveItr {
    curr: Board,
    next: Board,
    pos: Index2D,
    nr: i32
}

impl Iterator for MoveItr {
    fn next() -> Option<Board> {

    }
}

#[derive(Itrator)]
struct KingItr {
    curr: Board,
    next: Board,
    pos: Index2D,
    nr: i32
}

 impl Iterator for KingItr {
     fn new(board: Board, pos: Index2D) -> KingItr {
        KingItr {
            curr: board,
            next: None,
            pos,
            nr: 1
        }
     }

     fn next() -> Option<Board> {
        match  nr{
            1 => {
                next_move(Index2D{ x: pos.x, y: pos.y + 1 }, 3)
            }
            2 => {
                next_move(Index2D{ x: pos.x + 1, y: pos.y + 1 }, 1)
            }
            3 => {
                next_move(Index2D{ x: pos.x - 1, y: pos.y + 1 }, 1)
            }
            4 => {
                next_move(Index2D{ x: pos.x, y: pos.y - 1 }, 3)
            }
            5 => {
                next_move(Index2D{ x: pos.x + 1, y: pos.y - 1 }, 1)
            }
            6 => {
                next_move(Index2D{ x: pos.x - 1, y: pos.y - 1}, 1)
            }
            7 => {
                next_move(Index2D{ x: pos.x - 1, y: pos.y }, 1)
            }
            8 => {
                next_move(Index2D{ x: pos.x + 1, y: pos.y }, 1)
            }
            _ => None
        }
     }

     fn next_move(new_pos: Index2D, inc: i32) {
         if is_out_of_board(new_pos) {
             nr += inc;
             next()
         }
         if is_legal_move(curr, pos, new_pos) {
             nr += 1;
             some(create_new_board(curr, pos, new_pos))
         }
         else {
             nr += 1;
             next()
         }
     }
 }

pub fn create_new_board(board: Board, from: Index2D, to: Index2D) -> Board {
    let mut board = board;
    board.pieces[to.y][to.x] = [from.y][from.x];
    board.pieces[from.y][from.x] = Piece.Empty;
    board.turn = get_next_turn(board.turn);

    let board = board;
    board
}

pub fn is_legal_move(board: Board, from: Index2d, to: Index2D) -> bool {
    board[to.y][to.x].kind == Kind.Empty || board[to.y][to.x].color == board.turn
}

pub fn is_out_of_board(new_pos:Index2D) -> bool {
    let (x, y) = new_pos;

    if x < 0 || x > 8 ||
        y < 0 || y > 8 {
        true
    } else {
        false
    }
}

pub fn get_next_turn(color: Color) -> Color {
    if color = Color.White {
        Color.Black
    }else {
        Color.White
    }
}