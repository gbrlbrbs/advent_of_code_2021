use std::convert::TryInto;

use advent_of_code_2021 as aoc;

fn main() {
    let filepath = aoc::get_filepath(3);
    if filepath.exists() {
        let path_str = filepath.to_str()
            .expect("Error converting path to string");
        let file_str = aoc::read_file_to_string(path_str);
        let num_lines = aoc::lines(&file_str);
        let power_pt1 = part1(&file_str, num_lines);
        println!("Power comsumption is {}", power_pt1);
    } else {
        println!("File does not exist!");
    }
}

fn part1(file_str: &str, num_lines: usize) -> u32 {
    let mut line_itr = file_str.lines().peekable();
    let num_len = (*line_itr.peek().expect("Empty file!")).len();
    let mut count_vec: Vec<u32> = vec![0; num_len];
    for line in file_str.lines() {
        for (i, digit) in line.chars().enumerate() {
            if digit == '1' {
                count_vec[i] += 1;
            }
        }
    }
    let gamma_str = count_vec.into_iter()
        .map(|item| {
            if item > (num_lines/2).try_into().unwrap() {
                return '1';
            } else {
                return '0';
            }
        }).fold(String::new(), accumulate_str);
    let eps_str = gamma_str.chars()
        .map(|ch| {
            if ch == '1' {
                return '0'
            } else {
                return '1';
            }
        }).fold(String::new(), accumulate_str);
    let gamma = u32::from_str_radix(&gamma_str, 2).unwrap();
    let eps = u32::from_str_radix(&eps_str, 2).unwrap();
    gamma * eps
}

fn accumulate_str(mut accum: String, ch: char) -> String {
    accum.push(ch);
    accum
}