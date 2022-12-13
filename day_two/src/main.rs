//             OPPONENT   USER
// Rock     1     A        X
// Paper    2     B        Y
// Scissors 3     C        Z

extern crate core;


use std::fs::File;
use std::io::{BufRead, BufReader};

enum RoundScore {
    Win = 6,
    Tie = 3,
    Loss = 0,
}

enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn decode(letter: Option<char>) -> Shape{
    match letter {
        Some('A') =>Shape::Rock,
        Some('B')=>Shape::Paper,
        Some('C')=>Shape::Scissors,
        Some('X')=>Shape::Rock,
        Some('Y')=>Shape::Paper,
        Some('Z')=>Shape::Scissors,
        _=> panic!("Invalid character")
    }
}

fn decode_part_two(letter: Option<char>) -> RoundScore{
    match letter {
        Some('X')=>RoundScore::Loss,
        Some('Y')=>RoundScore::Tie,
        Some('Z')=>RoundScore::Win,
        _=>panic!("Invalid character")
    }
}



// Win 6
// Tie 3
// Draw 0
fn file_to_vector(file_name:&str)->Vec<Vec<String>>{
    let file = File::open(file_name).expect("EXPECTED A VALID FILE NAME");
    let reader = BufReader::new(file);

    let mut vector:Vec<Vec<String>> = Vec::new();
    for line in reader.lines(){
        let unwrapped_line = line.unwrap();
        let split_lines:Vec<&str> = unwrapped_line.split(" ").collect();
        let mut inner_vector:Vec<String> = Vec::new();
        inner_vector.push(split_lines[0].to_string());
        inner_vector.push(split_lines[1].to_string());
        vector.push(inner_vector);
    }
    return vector;
}

fn score_calculator(game:&Vec<String>)->i32{
    let opponent= decode(game[0].to_string().chars().nth(0));
    let user = decode(game[1].to_string().chars().nth(0));

    match user {
        Shape::Rock => match opponent {
            Shape::Rock => user as i32 + RoundScore::Tie as i32,
            Shape::Paper => user as i32 + RoundScore::Loss as i32,
            Shape::Scissors => user as i32 + RoundScore::Win as i32,
            _ => panic!("INVALID")
        }
        Shape::Paper => match opponent {
            Shape::Rock => user as i32 + RoundScore::Win as i32,
            Shape::Paper => user as i32 + RoundScore::Tie as i32,
            Shape::Scissors => user as i32 + RoundScore::Loss as i32,
            _ => panic!("INVALID")
        }
        Shape::Scissors => match opponent {
            Shape::Rock => user as i32 + RoundScore::Loss as i32,
            Shape::Paper => user as i32 + RoundScore::Win as i32,
            Shape::Scissors => user as i32 + RoundScore::Tie as i32,
            _ => panic!("INVALID")
        }
    }
}

fn score_calculator_part_two(game:&Vec<String>)->i32{
    let opponent= decode(game[0].to_string().chars().nth(0));
    let round_status = decode_part_two(game[1].to_string().chars().nth(0));

    match opponent {
        Shape::Rock => match round_status {
            RoundScore::Win => round_status as i32 + Shape::Paper as i32,
            RoundScore::Tie => round_status as i32 + Shape::Rock as i32,
            RoundScore::Loss => round_status as i32 + Shape::Scissors as i32,
            _ => panic!("INVALID")
        }
        Shape::Paper => match round_status {
            RoundScore::Win => round_status as i32 + Shape::Scissors as i32,
            RoundScore::Tie => round_status as i32 + Shape::Paper as i32,
            RoundScore::Loss => round_status as i32 + Shape::Rock as i32,
            _ => panic!("INVALID")
        }
        Shape::Scissors => match round_status {
            RoundScore::Win => round_status as i32 + Shape::Rock as i32,
            RoundScore::Tie => round_status as i32 + Shape::Scissors as i32,
            RoundScore::Loss => round_status as i32 + Shape::Paper as i32,
            _ => panic!("INVALID")
        }
    }
}

fn main() {
    // let file_name = "day_two_sample_input_data.txt";
    let file_name = "day_two_input_data.txt";
    let vector_of_games = file_to_vector(file_name);
    let mut overall_score: i32 = 0;
    for game in &vector_of_games {
        overall_score = overall_score + score_calculator(game);
    }
    let mut overall_score_two: i32 = 0;
    for game in &vector_of_games {
        overall_score_two = overall_score_two + score_calculator_part_two(game);
    }

    println!("PART ONE: ");
    println!("|-> OVERALL SCORE PART ONE : {overall_score}");
    println!("|-> OVERALL SCORE PART TWO : {overall_score_two}");
}
