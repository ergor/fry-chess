
extern crate termcolor;

#[cfg(test)]
mod tests;

mod gameset;
use gameset::{board::Board, piece::Color, position::Position};
use gameset::piece_defs;

use std::io;
use std::io::Write;
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

type SqColor = (Option<termcolor::Color>, Option<termcolor::Color>);

const WHITE_SQ: SqColor = (Some(termcolor::Color::Black), Some(termcolor::Color::White));
const BLACK_SQ: SqColor = (Some(termcolor::Color::White), Some(termcolor::Color::Black));

fn main() {
    let mut starter_board = gameset::generate_starting_board();

    starter_board.insert(piece_defs::from_def(piece_defs::rook::def(), Color::WHITE, Position::new(3, 5)));
    starter_board.insert(piece_defs::from_def(piece_defs::knight::def(), Color::WHITE, Position::new(5, 7)));

    print_board(&starter_board);

    for piece in starter_board.pieces().filter(|p| {p.color == Color::WHITE}) {
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
