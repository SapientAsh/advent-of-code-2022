use std::{fs::File, io::{BufReader, BufRead}, collections::{HashSet, HashMap}};

#[derive(Debug)]
struct Bag {
    compartment1: String,
    compartment2: String,
}

fn points(item: char) -> u32 {
    let ascii = item as u32;
    if ascii >= 65 && ascii <= 90 {
        return ascii - 38;
    }
    if ascii >= 97 && ascii <= 122 {
        return ascii - 96;
    }
   return 0;
}

fn main() -> std::io::Result<()> {
    let file = File::open("SecondaryFiles/fake_input.txt")?;
    let reader = BufReader::new(file);

    let bags: Vec<Bag> = reader.lines()
    .map( | line | {
        let line = line.unwrap();
        Bag {
            compartment1: String::from(&line[..line.len()/2]),
            compartment2: String::from(&line[line.len()/2..]),
        }
    })
    .collect();

    let part1: u32 = bags.iter()
    .map( | item | {
        let mut seen = HashSet::new();
        item.compartment1.chars()
        .fold( 0,| acc, letter1 | {
            acc + item.compartment2.chars() 
            .fold( 0,| acc, letter2 | {
                acc + match letter1 == letter2 && !seen.contains(&letter1) {
                    true => {
                        seen.insert(letter1);
                        points(letter1)
                    }
                    false => 0,
                }
            })
        })
    })
    .sum();

    let mut count = 0;
    let mut seen = HashMap::new();
    let part2: u32 = bags.iter()
    .map( | item | {
        count += 1;
        if count % 3 == 0 {
            seen = HashMap::new();
        }
        item.compartment1.chars()
        .fold( 0, | acc, letter1 | {
            acc + item.compartment2.chars()
            .fold( 0, | acc, letter2 | {
                acc + match letter1 == letter2 {
                    true => {
                        let times = seen.entry(letter1).or_insert(0);
                        *times += 1;
                        if count % 3 == 0 && seen.get(&letter1).copied().unwrap_or(0) == 3 {
                            points(letter1)
                        }
                        else { 0 }
                    }
                    false => 0,
                }
            })
        })
    })
    .sum();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}
