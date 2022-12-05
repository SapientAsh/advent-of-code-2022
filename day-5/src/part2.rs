use std::{fs::File, io::{BufReader, BufRead}};

pub struct Move {
    pub qty: i32,
    pub src: usize,
    pub det: usize,
}

impl Move {
    pub fn new(source: &str) -> Move {
        let line: Vec<&str> = source.split(" ").collect();
        Move {
            qty: line[1].parse::<i32>().unwrap(),
            src: line[3].parse::<usize>().unwrap(),
            det: line[5].parse::<usize>().unwrap(),
        }
    }
}

pub fn main() -> String {
    let file = File::open("SecondaryFiles/input.txt").unwrap();
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
        let mut stack = Vec::new();
        for _ in 0..instruction.qty {
            let crate_ = rev_crates[instruction.src - 1].pop().unwrap();
            stack.push(crate_);
        }
        rev_crates[instruction.det  - 1].append(
            &mut stack.into_iter().rev().collect()
        );
    }

    let part2: String = rev_crates.iter().map( | stack | {
        *stack.last().unwrap_or(&&' ')
    })
    .collect();

    part2
}