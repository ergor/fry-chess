
extern crate termcolor;

mod gameset;
use gameset::{Board, Color, Position};
use gameset::piece_defs;

use std::io;
use std::io::Write;
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    let mut starter_board = gameset::generate_starting_board();

    let rook_pos = Position::new(3, 5);
    let rook = piece_defs::from_def(piece_defs::rook::def(), &starter_board, Color::WHITE, rook_pos);
    starter_board.insert(rook);

    print_board(&starter_board);

    for piece in starter_board.into_iter() {
        for possible_boards in piece.into_iter() {

        }
    }

    // IDE: lagre brikkene i en hashmap? key: posisjon. value: brikken.
}

fn print_board_files() {
    print!("   ");
    for c in (b'a'..=b'h').map(char::from) {
        print!(" {} ", c);
    }
    println!("");
}

fn print_board<'a>(board: &Board<'a>) {

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
            // print ranks
            //if x == -1 || x == 8 {
            //    print!(" {} ", (7-y) + 1);
            //    continue;
            //}
            // print the squares
            print_sq(
                if (x+y) & 1 == 0 {white_sq} else {black_sq},
                match board.piece_at(Position::new(x, y)) {
                    Some(piece) => piece.character(),
                    None => ' '
                }
            ).unwrap();
        }
        println!("");
    }

    print_board_files();
}

