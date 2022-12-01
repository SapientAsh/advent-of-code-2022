use std::{fs::File, io::{BufReader, BufRead}};

struct Elf {
    calories: i32,
}

fn main() -> std::io::Result<()> {
    let file = File::open("SecondaryFiles/input.txt")?;
    let reader = BufReader::new(file);

    let mut elves: Vec<Elf> = Vec::new();
    let mut sum = 0;
    for wrapped_line in reader.lines() {
        let line = wrapped_line?;
        if line == "" {
            elves.push( Elf {
                calories: sum
            });
            sum = 0;
        }
        else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    let most_calories = elves.iter()
    .reduce( |accum, item| {
        if accum.calories >= item.calories { accum } else { item }
    }).unwrap()
    .calories;

    println!("{most_calories} calories");

    Ok(())
}
