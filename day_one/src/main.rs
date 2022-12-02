
use std::fs::File;
use std::io::{BufReader, prelude::*};


fn file_to_vector(file_name: &str)->Vec<i32>{
    let file = File::open(file_name).expect("expected a valid file");
    let reader = BufReader::new(file);

    let mut vector: Vec<i32> = Vec::new();
    let mut max_amount = 0;
    for line in reader.lines(){
        let unwrapped_line = line.unwrap();
        if unwrapped_line.is_empty() {
            vector.push(max_amount);
            max_amount = 0;
        }else{
            let number: i32 = unwrapped_line.parse().unwrap();
            max_amount = max_amount + number;
        }
    }
    vector
}


fn part_one(file_name: &str) -> i32{
    let mut summarized_calories_per_elf = file_to_vector(file_name);
    summarized_calories_per_elf.sort();
    summarized_calories_per_elf.reverse();
    summarized_calories_per_elf[0]
}

fn part_two(file_name:&str)->i32{
    let mut summarized_calories_per_elf = file_to_vector(file_name);
    summarized_calories_per_elf.sort();
    summarized_calories_per_elf.reverse();
    return summarized_calories_per_elf[0] + summarized_calories_per_elf[1] + summarized_calories_per_elf[2];
}

fn main() {
    let file_name = "day_one_input_data.txt";
    let answer_one = part_one(file_name);
    let answer_two = part_two(file_name);
    println!("DAY ONE");
    println!("|-> Part One: {answer_one}");
    println!("|-> Part Two: {answer_two}");
}
