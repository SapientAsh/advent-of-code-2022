use std::{fs::File, io::{BufReader, BufRead, Lines}};

//struct Directory {
//    size: i32,
//}

fn directory_size(reader: &mut Lines<BufReader<File>>, list: &mut Vec<i32>) -> i32 {
    let mut size: i32 = 0;
    //let name = String::from(reader.next()
    //.unwrap().unwrap()
    //.split(" ")
    //.collect::<Vec<&str>>()[1]);

    loop {
        let line = reader.next();
        match line {
            None => break,
            _ => (),
        };

        let line = line.unwrap().unwrap();
        if line == "$ cd .." {
            break;
        }
        let words: Vec<&str> = line.split(" ").collect();
        if words[1] == "cd" {
            size += directory_size(reader, list);
        }
        size += words[0].parse::<i32>().unwrap_or(0);
    }
    list.push(size);

    size
}

fn main() -> std::io::Result<()> {
    let file = File::open("SecondaryFiles/input.txt")?;
    let mut reader = BufReader::new(file).lines();
    let mut directories: Vec<i32> = Vec::new();
    let total = directory_size(&mut reader, &mut directories);

    let part1: i32 = directories.iter()
    .fold(0, | acc, directory | {
        acc + if *directory <= 100000 { *directory } else { 0 }
    });

    let upgrade_size = 30000000;
    let max_space = 70000000;
    let target_space = upgrade_size - (max_space - total);

    let part2: i32 = directories.iter()
    .fold(total, | acc, directory | {
        if *directory <= acc && *directory >= target_space { *directory } else { acc }
    });

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}
