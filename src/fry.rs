
extern crate termcolor;

/*
#[cfg(test)]
mod tests;
*/
mod gameset;
mod math;

use math::*;
use math::Position;
use gameset::*;
use gameset::piece_defs::*;

use std::env;
use std::io;
use std::io::Write;
use std::io::BufRead;
use std::fs::File;

use san_rs::Move;
use fen::BoardState;
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

type SqColor = (Option<termcolor::Color>, Option<termcolor::Color>);

const DEFAULT_FEN: &'static str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const WHITE_SQ: SqColor = (Some(termcolor::Color::Black), Some(termcolor::Color::White));
const BLACK_SQ: SqColor = (Some(termcolor::Color::White), Some(termcolor::Color::Black));

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut fen = String::new();

    if args.len() > 1 {
        let file = File::open(&args[1]).unwrap();
        let mut reader = io::BufReader::new(file);
        reader.read_line(&mut fen).unwrap();
    } else {
        fen.push_str(DEFAULT_FEN);
    }

    let board_state = BoardState::from_fen(&fen).unwrap();
    let starter_board = Board::from_state(board_state);

    print_board(&starter_board);

    let mut i = 0;
    loop {
        println!("ply: {}, move: {}", i+1, (i/2) + 1);
        let mov = player_move();
        if let Err(e) = mov {
            println!("invalid input");
            continue;
        }
        let mov = Move::parse(&mov.unwrap());
        println!("{:?}", mov);

        i += 1;
    }

    //for moving_piece in starter_board.pieces().filter(|p| {p.color == Color::White}) {
    //    for board in starter_board.generate(moving_piece) {
    //        print_board(&board);
    //    }
    //}


}

fn player_move() -> io::Result<String> {
    print!("player> ");
    io::stdout().flush() ?;
    let mut input = String::new();
    io::stdin().read_line(&mut input) ?;
    input.pop();
    Ok(input)
}

fn print_board_files() {
    print!("   ");
    for c in (b'a'..=b'h').map(char::from) {
        print!(" {} ", c);
    }
    println!("");
}

fn print_board_rank(y: usize) {
    print!(" {} ", (7-y) + 1);
}

pub fn print_sq(st: &mut StandardStream, c: char, colors: SqColor) -> io::Result<()> {
        st.set_color(ColorSpec::new().set_fg(colors.0).set_bg(colors.1))?;
        write!(st, " {} ", c)?;
        st.reset()
}

pub fn print_board(board: &Board) {

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    print_board_files();
    for y in 0..8 {
        for x in 0..8 {
            // print rank on left side
            if x == 0 {  print_board_rank(y); }
            // print the squares
            print_sq(
                &mut stdout,
                match board.piece_at(Position::new(x, y)) {
                    Some(piece) => piece.character(),
                    None => ' '
                },
                if (x+y) & 1 == 0 {WHITE_SQ} else {BLACK_SQ}
            ).unwrap();
            // print rank on right side
            if x == 7 { print_board_rank(y); }
        }
        println!("");
    }
    print_board_files();
}
