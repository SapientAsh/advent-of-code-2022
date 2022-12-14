use std::{fs::File, io::{BufReader, BufRead}};

pub mod part2;
use part2::Move;

fn main() -> std::io::Result<()> {
    let file = File::open("SecondaryFiles/input.txt")?;
    let reader = BufReader::new(file);

    let mut reader_iter = reader.lines();
    let mut crates: Vec<Vec<char>> = vec![Vec::new(); 9];
    for line in &mut reader_iter {
        let line = line.unwrap();
        if line.contains("1") {
            continue;
        }
        if line == "" {
            break;
        }
        for i in 0..9 {
            let content = line.chars().nth(4 * i + 1).unwrap();
            if content != ' '{
                crates[i].push(content);
            }
        }
    }
    
    let mut rev_crates = vec![Vec::new(); 9];
    for i in 0..9 {
        rev_crates[i] = crates[i].iter()
        .rev()
        .collect();
    }
    
    let moves: Vec<Move> = reader_iter
    .map( | line | {
        let line = line.unwrap();
        Move::new(&line)
    })
    .collect();

    for instruction in moves.iter() {
        for _ in 0..instruction.qty {
            let crate_ = rev_crates[instruction.src - 1].pop().unwrap();
            rev_crates[instruction.det  - 1].push(crate_);
        }
    }

    let part1: String = rev_crates.iter().map( | stack | {
        *stack.last().unwrap_or(&&' ')
    })
    .collect();

    let part2 = part2::main();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}
