use std::fs;

pub fn result() -> u64 {
    let input = fs::read_to_string("src/day3/input.txt").unwrap();
    parse_input(&input)
}

fn parse_input(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mid = line.char_indices().count() / 2;
            let mid_byte = line.char_indices().nth(mid).map_or(line.len(), |(i, _)| i);
            let (left, right) = line.split_at(mid_byte);
            let letter = find_letter(left, right).unwrap();
            convert_to_priority(letter)
        })
        .sum()
}

fn find_letter(container_1: &str, container_2: &str) -> Option<char> {
    for letter in container_1.chars() {
        if container_2.contains(letter) {
            return Some(letter);
        }
    }
    None
}

fn convert_to_priority(letter: char) -> u64 {
    let unicode = letter as u64;
    if unicode > 90 {
        unicode - 96
    } else {
        unicode - 64 + 26 
    }
}