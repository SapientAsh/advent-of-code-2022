use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

#[derive(Debug)]
struct Bag {
    compartment1: String,
    compartment2: String,
}

fn priority(item: char) -> u32 {
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
    let file = File::open("SecondaryFiles/input.txt")?;
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
                        priority(letter1)
                    }
                    false => 0,
                }
            })
        })
    })
    .sum();

    let file = File::open("SecondaryFiles/input.txt")?;
    let reader = BufReader::new(file);

    let mut index = 0;
    let mut buffer: Vec<HashSet<char>> = Vec::new();
    let part2: u32 = reader.lines()
    .map( | line | {
        index += 1;
        let line = line.unwrap();
        let mut output = 0;
        let mut chars = HashSet::new();
        line.chars()
        .for_each( | letter | {
            chars.insert(letter);
        });
        buffer.push(chars);
        if index % 3 == 0 {
            let mut temp = HashSet::new();
            buffer[2].intersection(&buffer[1])
            .for_each( | letter | {
                temp.insert(*letter);
            });
            let badge: &char = temp.intersection(&buffer[0])
            .collect::<Vec<&char>>()
            .first()
            .unwrap();
            output = priority(*badge);
            buffer = Vec::new();
        }
        output
    })
    .sum();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}
