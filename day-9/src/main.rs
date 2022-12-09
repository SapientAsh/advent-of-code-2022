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

fn distance(h: Point, t: Point) -> f32 {
    ((h.x - t.x).pow(2) as f32 + (h.y - t.y).pow(2) as f32).ceil()
}

fn evaluate_move((head, tail): (&mut Point, &mut Point), dir: &Move, set: &mut HashSet<Point>) {
    for _ in 0..dir.distance {
        match dir.direction {
            Direction::Up => head.y += 1,
            Direction::Down => head.y -= 1,
            Direction::Left => head.x -= 1,
            Direction::Right => head.x += 1,
        }

        if head.y - tail.y > 1 && head.x == tail.x { tail.y += 1 }
        else if head.y - tail.y < -1 && head.x == tail.x { tail.y -= 1 }
        else if head.x - tail.x > 1 && head.y == tail.y { tail.x += 1 }
        else if head.x - tail.x < -1 && head.y == tail.y { tail.x -= 1 }
        else if distance(*head, *tail) > 2.0 {
            if head.y > tail.y && head.x > tail.x {
                tail.x += 1;
                tail.y += 1;
            }
            else if head.y > tail.y && head.x < tail.x {
                tail.x -= 1;
                tail.y += 1;
            }
            else if head.y < tail.y && head.x > tail.x {
                tail.x += 1;
                tail.y -= 1;
            }
            else if head.y < tail.y && head.x < tail.x {
                tail.x -= 1;
                tail.y -= 1;
            }
        }

        set.insert(*tail);
    }
}

fn main() -> std::io::Result<()> {
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

    let mut visited = HashSet::<Point>::new();
    let (mut head, mut tail) = (
        Point { x: 0, y: 0 },
        Point { x: 0, y: 0 },
    );

    moves.iter()
    .for_each( | dir | {
        evaluate_move((&mut head, &mut tail), dir, &mut visited);
    });

    let part1 = visited.len();
    println!("Part1: {part1}");

    Ok(())
}
