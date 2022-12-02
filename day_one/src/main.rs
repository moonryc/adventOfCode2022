use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str::FromStr;





fn part_one(file_name: &str) -> i32{
    let file = File::open(file_name).expect("expected a valid file");
    let reader = BufReader::new(file);

    let mut currentIndex = 0;
    let mut maxIndex = 0;
    let mut maxTotal:i32 = 0;
    let mut currentTotal = 0;
    for line in reader.lines(){
        let unwrapped_line = line.unwrap();
        if unwrapped_line.is_empty() {
            if currentTotal > maxTotal {
                maxTotal = currentTotal;
                maxIndex = currentIndex;
            }
            currentIndex = currentIndex +1;
            currentTotal = 0;
        } else{
            let guess: i32 = match unwrapped_line.trim().parse(){
                Ok(num)=> num,
                Err(_)=>continue,
            };
            currentTotal = currentTotal + guess;
        }

    }
    return maxTotal;
}

fn part_two(file_name:&str)->i32{
    let file = File::open(file_name).expect("expected a valid file");
    let reader = BufReader::new(file);

    let mut vector: Vec<i32> = Vec::new();
    let mut maxAmount = 0;
    for line in reader.lines(){
        let unwrapped_line = line.unwrap();
        if unwrapped_line.is_empty() {
            vector.push(maxAmount);
            maxAmount = 0;
        }else{
            let number: i32 = unwrapped_line.parse().unwrap();
           maxAmount = maxAmount + number;
        }
    }
    vector.sort();
    vector.reverse();
    return vector[0] + vector[1] + vector[2];
}

fn main() {
    let file_name = "day_one_input_data.txt";
    let answer_one = part_one(file_name);
    let answer_two = part_two(file_name);
    println!("DAY ONE");
    println!("    Part one: {answer_one}");
    println!("    part two: {answer_two}");
}
