
extern crate termcolor;

mod gameset;
use gameset::{board::Board, piece::Color, position::Position};
use gameset::piece_defs;

use std::io;
use std::io::Write;
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    let mut starter_board = gameset::generate_starting_board();

    let rook_pos = Position::new(3, 5);
    let rook = piece_defs::from_def(piece_defs::rook::def(), Color::WHITE, rook_pos);
    starter_board.insert(rook);

    print_board(&starter_board);

    for piece in starter_board.pieces() {
        for possible_board in piece.generator(&starter_board) {
            print_board(&possible_board);
        }
    }
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

fn print_board(board: &Board) {

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    let white_sq = (Some(termcolor::Color::Black), Some(termcolor::Color::White));
    let black_sq = (Some(termcolor::Color::White), Some(termcolor::Color::Black));

    let mut print_sq = | colors: (Option<termcolor::Color>, Option<termcolor::Color>),
                         c: char |
                       -> io::Result<()> {
        stdout.set_color(ColorSpec::new().set_fg(colors.0).set_bg(colors.1))?;
        write!(stdout, " {} ", c)?;
        stdout.reset()
    };

    print_board_files();
    for y in 0..8 {
        for x in 0..8 {
            // print rank on left side
            if x == 0 {  print_board_rank(y); }
            // print the squares
            print_sq(
                if (x+y) & 1 == 0 {white_sq} else {black_sq},
                match board.piece_at(Position::new(x, y)) {
                    Some(piece) => piece.character(),
                    None => ' '
                }
            ).unwrap();
            // print rank on right side
            if x == 7 { print_board_rank(y); }
        }
        println!("");
    }
    print_board_files();
}

