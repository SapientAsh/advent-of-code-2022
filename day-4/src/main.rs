use std::{fs::File, io::{BufReader, BufRead}, str::Split};

struct Pair {
    elf1: (i32, i32),
    elf2: (i32, i32),
}

fn main() -> std::io::Result<()> {
    let file = File::open("SecondaryFiles/input.txt")?;
    let reader = BufReader::new(file);

    let pairs: Vec<Pair> = reader.lines()
    .map( | line | {
        let line = line.unwrap();
        let elves: Vec<&str>  = line.split(",").collect();
        let mut split: (Split<&str>, Split<&str>) = (elves[0].split("-"), elves[1].split("-"));
        Pair {
            elf1: (split.0.next().unwrap().parse::<i32>().unwrap(), split.0.next().unwrap().parse::<i32>().unwrap()),
            elf2: (split.1.next().unwrap().parse::<i32>().unwrap(), split.1.next().unwrap().parse::<i32>().unwrap()),
        }
    })
    .collect();

    let part1: i32 = pairs.iter().fold( 0, |acc, pair | {
        acc + if (pair.elf1.0 >= pair.elf2.0 && pair.elf1.1 <= pair.elf2.1) || (pair.elf2.0 >= pair.elf1.0 && pair.elf2.1 <= pair.elf1.1) { 1 }
        else { 0 }
    });

    let part2: i32 = pairs.iter().fold( 0, |acc, pair | {
        acc + if 
        (pair.elf1.0 >= pair.elf2.0 && pair.elf1.0 <= pair.elf2.1) || 
        (pair.elf1.1 >= pair.elf2.0 && pair.elf1.1 <= pair.elf2.1) ||
        (pair.elf1.0 <= pair.elf2.0 && pair.elf1.1 >= pair.elf2.1) { 1 }
        else { 0 }
    });

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}
