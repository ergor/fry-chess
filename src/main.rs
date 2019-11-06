
#[derive(Copy, Clone)]
enum Color{
    White,
    Black
}

#[derive(Copy, Clone)]
struct Board {
    squares: [[i32; 8]; 8],
    turn: Color
}

#[derive(Copy, Clone)]
struct Point {
    x:i32,
    y:i32,
    piece:i32
}

#[derive(Copy, Clone)]
struct Board2 {
    squares: [point; 64],
    turn: Color
}



fn main() {
    let board1 = Board{
        squares : [
            [0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,0,0],
        ],
        turn: Color::White
    };


    println!("Hello, world!");

}
