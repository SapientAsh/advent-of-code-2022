use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Move {
    direction: Direction,
    distance: i32,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point {
            x: x,
            y: y,
        }
    }
}

fn distance(h: Point, t: Point) -> f32 {
    ((h.x - t.x).pow(2) as f32 + (h.y - t.y).pow(2) as f32).ceil()
}

fn move_rope(rope: &mut [Point], set: &mut [HashSet::<Point>]) {
    if rope[0].y - rope[1].y > 1 && rope[0].x == rope[1].x { rope[1].y += 1 }
    else if rope[0].y - rope[1].y < -1 && rope[0].x == rope[1].x { rope[1].y -= 1 }
    else if rope[0].x - rope[1].x > 1 && rope[0].y == rope[1].y { rope[1].x += 1 }
    else if rope[0].x - rope[1].x < -1 && rope[0].y == rope[1].y { rope[1].x -= 1 }
    else if distance(rope[0], rope[1]) > 2.0 {
        if rope[0].y > rope[1].y && rope[0].x > rope[1].x {
            rope[1].x += 1;
            rope[1].y += 1;
        }
        else if rope[0].y > rope[1].y && rope[0].x < rope[1].x {
            rope[1].x -= 1;
            rope[1].y += 1;
        }
        else if rope[0].y < rope[1].y && rope[0].x > rope[1].x {
            rope[1].x += 1;
            rope[1].y -= 1;
        }
        else if rope[0].y < rope[1].y && rope[0].x < rope[1].x {
            rope[1].x -= 1;
            rope[1].y -= 1;
        }
    }

    set[1].insert(rope[1]);
    if rope.len() > 2 {
        move_rope(&mut rope[1..], &mut set[1..])
    }
}

fn main() -> std::io::Result<()> {
    const ROPE_LENGTH: usize = 10;

    let file = File::open("SecondaryFiles/input.txt")?;
    let reader = BufReader::new(file);

    let moves: Vec<Move> = reader.lines()
    .map( | line | {
        let line = line.unwrap();
        let dir: Vec<&str> = line.split(" ").collect();
        Move { 
            direction: match dir[0] {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid input!"),
            },
            distance: dir[1].parse::<i32>().unwrap(),
        }

    })
    .collect();

    let mut rope = vec![Point::new(0, 0); ROPE_LENGTH];
    let mut visited = vec![HashSet::<Point>::new(); ROPE_LENGTH];

    moves.iter()
    .for_each( | dir | {
        for _ in 0..dir.distance {
            match dir.direction {
                Direction::Up => rope[0].y += 1,
                Direction::Down => rope[0].y -= 1,
                Direction::Left => rope[0].x -= 1,
                Direction::Right => rope[0].x += 1,
            }
            move_rope(&mut rope, &mut visited);
        }
    });

    let part1 = visited[ROPE_LENGTH - 1].len();
    println!("Part 1: {part1}");

    Ok(())
}
