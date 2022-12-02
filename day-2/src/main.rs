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

fn main() -> std::io::Result<()> {
    let file = File::open("SecondaryFiles/input.txt")?;
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

    println!("Total points = {points}");

    Ok(())
}
