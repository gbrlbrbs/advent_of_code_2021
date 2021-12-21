use advent_of_code_2021 as aoc;
use std::path::Path;

fn main() {
    let filepath = Path::new("data/input_day1.txt");
    let path = filepath.canonicalize()
        .expect("Error canonicalizing");
    if path.exists() {
        let path_str = path.to_str()
            .expect("Error getting path");
        let file_str = aoc::read_file_to_string(path_str);
        let numbers = aoc::read_string_separate_lines(&file_str);
        let mut number_vec: Vec<i32> = numbers.into_iter()
            .map(|num| {
                num.parse::<i32>().unwrap()
        }).collect();
        let mut counter = 0;
        while !number_vec.is_empty() {
            let last = number_vec.pop().expect("Vector emptied unexpectedly");
            if !number_vec.is_empty() && 
                (*(number_vec.last().expect("Vector exhausted unexpectedly")) < last) {
                counter += 1;
            }
        }
        println!("There are {} positive changes in depth.", counter);
    } else {
        println!("File does not exist!");
    }
}