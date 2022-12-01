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

    elves.sort_by_key( | e | e.calories);
    let elves: Vec<&Elf> = elves.iter().rev().collect();

    println!("The elves with the most calories have {}, {}, and {} calories for a total of {}", elves[0].calories, elves[1].calories, elves[2].calories, elves[0].calories + elves[1].calories + elves[2].calories);

    Ok(())
}
