use std::io;

fn main() {
    println!("Hello world");

    let value = func(5, 6);

    let mut input = String::new();

    let read_size = match io::stdin().read_line(&mut input) {

        Ok(5) => {
            println!("PATTERN MATCHING YAY");
            5
        },
        Ok(size) => size,
        Err(e) => { 
            println!("an error occured: {}", e);
            0
        }
    };

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;


    println!("read size: {}\ndata: {}", read_size, input);
}

fn func(x: i32, y: i32) -> i32 {
    x + y
}