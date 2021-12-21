use advent_of_code_2021 as aoc;
use aoc::day2::Directions;

fn main() {
    let filepath = aoc::get_filepath(2);
    if filepath.exists() {
        let path_str = filepath.to_str()
            .expect("Error converting path to string");
        let file_str = aoc::read_file_to_string(path_str);
        let mut forward = 0;
        let mut down = 0;
        for line in file_str.lines() {
            let dir = aoc::day2::find_direction(line);
            match dir {
                Directions::Forward(x) => forward += x,
                Directions::Down(y) => down += y
            };
        }
        println!("Submarine is in position ({}, {}), which multiplied is {}", forward, down, forward * down);
    } else {
        println!("File does not exist!");
    }
}