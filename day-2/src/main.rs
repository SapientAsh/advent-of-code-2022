use std::{fs::File, io::{BufReader, BufRead}};

struct Match {
    player: Move,
    opponent: Move,
}

#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn part1() -> i32 {
    fn evaluate_points(game: &Match) -> i32 {
        let mut points = 0;
        points += match game.player {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        if game.player == game.opponent {
            points += 3;
        }
        else if 
        game.player == Move::Rock     && game.opponent == Move::Scissors ||
        game.player == Move::Scissors && game.opponent == Move::Paper    ||
        game.player == Move::Paper    && game.opponent == Move::Rock     {
            points += 6;
        }

        points
    }

    let file = File::open("SecondaryFiles/input.txt").unwrap();
    let reader = BufReader::new(file);

    let tournament: Vec<Match> = reader.lines()
    .map( | line | {
        let line = line.unwrap();
        let moves: Vec<&str> = line.split(" ").collect();
        let opponent: Move = {
            if moves[0] == "A" { Move::Rock }
            else if moves[0] == "B" { Move::Paper }
            else { Move::Scissors }
        };
        let player: Move = {
            if moves[1] == "X" { Move::Rock }
            else if moves[1] == "Y" { Move::Paper }
            else { Move::Scissors }
        };
        Match {
            player: player,
            opponent: opponent,
        }
    })
    .collect();

    let points = tournament.iter().fold(0, | acc, game | {
        acc + evaluate_points(game)
    });

    points
}

fn part2() -> i32 {
    let file = File::open("SecondaryFiles/input.txt").unwrap();
    let reader = BufReader::new(file);

    let points: i32 = reader.lines()
    .map( | line | {
        let line = line.unwrap();
        let moves: Vec<&str> = line.split(" ").collect();
        let opponent = {
            if moves[0] == "A" { 0 }
            else if moves[0] == "B" { 1 }
            else { 2 }
        };
        
        if moves[1] == "X" { 1 + ((opponent - 1) % 3 + 3) % 3 }
        else if moves[1] == "Y" { 4 + opponent }
        else { 7 + (opponent + 1) % 3 }
    })
    .collect::<Vec<i32>>()
    .iter()
    .sum();

    points
}

fn main() {
    let part1 = part1();
    let part2 = part2();

    println!("Part 1 = {part1}");
    println!("Part 2 = {part2}");
}
