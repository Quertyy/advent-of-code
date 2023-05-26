use core::panic;
use std::fs::File;
use std::io::{ self, BufRead };
use std::vec;

pub fn get_result() -> i32 {
    let file = File::open("src/day2/input.txt").unwrap();
    let reader = io::BufReader::new(file);
    
    let mut rounds_list= vec![];
    let mut round_result = vec![];

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let round = line.to_owned();
                let mut values = round.split_whitespace();
                if let (Some(player_1), Some(player_2)) = (values.next(), values.next()) {
                    round_result.push(player_1.to_string());
                    round_result.push(player_2.to_string());
                }
                rounds_list.push(round_result.clone());
                round_result.clear();
            },
            Err(_) => continue
        }
    }
    let mut total_score = 0;

    for element in rounds_list {
        let choice_player_1 = element[0].chars().next().unwrap();
        let choice_player_2 = element[1].chars().next().unwrap();
        let (_, choice_score, result_score) = choose_result(choice_player_1, choice_player_2);
        total_score += choice_score + result_score;

    }
    total_score
}

// return choice, choice_score, result_score
fn choose_result(choice_1: char, choice_2: char) -> (char, i32, i32) {
    match choice_2 {
        'X' => {
            match choice_1 {
                'A' => ('C', 3, 0),
                'B' => ('A', 1, 0),
                'C' => ('B', 2, 0),
                _ => panic!()
            }
        },
        'Y' => {
            match choice_1 {
                'A' => ('A', 1, 3),
                'B' => ('B', 2, 3),
                'C' => ('C', 3, 3),
                _ => panic!()
            }
        },
        'Z' => {
            match choice_1 {
                'A' => ('B', 2, 6),
                'B' => ('C', 3, 6),
                'C' => ('A', 1, 6),
                _ => panic!()
            }
        },
        _ => panic!("Invalid input for second choice"),
    }
}

// A est pierre
// B est papier
// C est ciseaux
// X est perdre
// Y est nul
// Z est gagner