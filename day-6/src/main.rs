use std::fs;

//Recursive function to determine whether a string slice is only unique characters
fn mutually_exclusive(src: &str) -> bool {
    if src.len() == 1 { return true; }

    let first = src.chars().nth(0).unwrap();
    for char in src[1..].chars() {
        if char == first { return false; }
    }

    return mutually_exclusive(&src[1..]);
}

//Finds the earliest index at which <size> previous characters are unique
fn find_seq(src: &str, size: usize) -> usize {
    let mut index: usize = size;
    while index < src.len() {
        if mutually_exclusive(&src[index - (size - 1)..=index]) { 
            break;
        }
        index += 1;
    };

    index + 1
}

fn main() -> std::io::Result<()> {
    let content = fs::read_to_string("SecondaryFiles/input.txt")?;
    let part1 = find_seq(&content, 4);
    let part2 = find_seq(&content, 14);

    println!("Part 1 = {part1}");
    println!("Part 2 = {part2}");

    Ok(())
}
