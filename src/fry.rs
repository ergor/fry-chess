
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    print_board();
}

fn print_board_files() {
    print!("   ");
    for c in (b'a'..b'i').map(char::from) {
        print!(" {} ", c);
    }
    println!("");
}

fn print_board() {
    let white_sq = (Some(Color::Black), Some(Color::White));
    let black_sq = (Some(Color::White), Some(Color::Black));

    fn print_sq(stream: &mut StandardStream, 
                colors: (Option<Color>, Option<Color>),
                c: char) {
        stream.set_color(ColorSpec::new().set_fg(colors.0)).expect("");
        stream.set_color(ColorSpec::new().set_bg(colors.1)).expect("");
        write!(stream, " {} ", c).expect("");
        stream.reset().expect("");
    }

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    print_board_files();
    for y in 0..8 {
        for x in -1..9 {

            if x == -1 || x == 8 {
                print!(" {} ", (7-y) + 1);
                continue;
            }

            if (x+y) & 1 == 0 {
                print_sq(&mut stdout, white_sq, ' ');
            } else {
                print_sq(&mut stdout, black_sq, ' ');
            }
        }
        println!("");
    }
    print_board_files();
}
