use std::{fs::File, io::{BufReader, BufRead}};

struct Move {
    qty: i32,
    src: i32,
    det: i32,
}

fn main() -> std::io::Result<()> {
    let file = File::open("SecondaryFiles/input.txt")?;
    let reader = BufReader::new(file);

    Ok(())
}
