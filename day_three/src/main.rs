use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn item_to_item_value(item:&char)->i32{
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut index = 1;
    for char in alphabet{
        if *item == char{
            break;
        }
        index = index +1;
    }
    index
}


fn find_duplicate(rucksack:String)->char{
    let mut rucksack_hashmap: HashMap<char, i32> = HashMap::new();

    let (bag_one, bag_two) = rucksack.split_at(rucksack.len()/2);

    for item in bag_one.chars(){
        rucksack_hashmap.insert(item, 1);
    }

    for item in bag_two.chars(){
        if rucksack_hashmap.contains_key(&item){
            return item
        }
    }

    return 'a'
}

fn get_points_from_rucksack(rucksack:String)->i32{
    let duplicate = find_duplicate(rucksack);
    let value =item_to_item_value(&duplicate);
    value
}


fn main() {
    let file_name = "day_three_input_data.txt";
    // let file_name = "day_three_sample_input_data.txt";
    let file = File::open(file_name).expect("Expected a valid file");
    let reader = BufReader::new(file);


    // let test = find_duplicate("vJrwpWtwJgWrhcsFMMfFFhFp".to_string());
    // println!("{test}")

    let mut total_points = 0;
    for line in reader.lines(){
        let unwrapped_line = line.unwrap();
        total_points = total_points + get_points_from_rucksack(unwrapped_line)
    }

    println!("DAY THREE");
    println!("|-> Part One: {total_points}");

}
