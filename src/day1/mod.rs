use std::fs::File;
use std::io::{ self, BufRead };
use std::str::FromStr;
use std::collections::HashMap;

pub fn get_top_three() -> HashMap<usize, i32> {
    let file = File::open("src/day1/input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let mut calories: Vec<i32> = Vec::new();
    let mut top_elves = HashMap::new();
    let mut elf_total: i32 = 0;

    for line in reader.lines() {
        match i32::from_str(&line.unwrap()) {
            Ok(number) => elf_total += number,
            Err(_) => {
                calories.push(elf_total);
                elf_total = 0;
            }
        }
    }

    for _ in 0..3 {
        if let Some((first_elf_value, first_elf_index)) = get_max(&calories) {
            top_elves.insert(first_elf_index, first_elf_value);
            calories.remove(first_elf_index);   
        }
    }
    top_elves
    
}

pub fn get_max(calories: &Vec<i32>) -> Option<(i32, usize)> {
    calories.iter().enumerate().max_by_key(|(_, &value)| value).map(|(index, &value)| (value, index))
}