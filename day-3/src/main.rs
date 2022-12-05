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
                        priority(letter1)
                    }
                    false => 0,
                }
            })
        })
    })
    .sum();

    let file = File::open("SecondaryFiles/fake_input.txt")?;
    let reader = BufReader::new(file);

    let mut count = 0;
    let mut seen = vec![HashSet::new()];
    let part2 = reader.lines()
    .fold( 0, | acc, line | {
        let line = line.unwrap();
        count += 1;
        seen.push(HashSet::new());
        let _: Vec<bool> = line.chars()
        .map( | letter | seen[count].insert(letter))
        .collect();
        if count % 3 == 0 {
            let mut temp = HashSet::new();
            let _: Vec<bool> = seen[count - 2].intersection(&seen[count - 1])
            .map( | item | temp.insert(*item))
            .collect();
            acc + temp.intersection(&seen[count])
            .fold( 0, | acc, item | {
                acc + priority(*item)
            })
        }
        else { 0 }
    });


    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}
