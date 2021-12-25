use std::fs;
use std::vec::Vec;
use std::path::{PathBuf, Path};

pub fn get_filepath(day: u32) -> PathBuf {
    let relative_path = format!("data/input_day{}.txt", day);
    Path::new(&relative_path).canonicalize()
        .expect("Error canonicalizing filepath")
}

pub fn read_file_to_string(path: &str) -> String {
    fs::read_to_string(path)
        .expect("Something went wrong when reading the file")
}

pub fn read_string_separate_lines(whole_string: &str) -> Vec<&str> {
    whole_string.lines().collect()
}

pub fn lines(file_str: &str) -> usize {
    file_str.lines().into_iter().count()
}

pub mod day2 {
    pub enum Directions {
        Forward(i32),
        Down(i32)
    }

    pub fn find_direction(line: &str) -> Directions {
        let mut data: Vec<&str> = line.split_whitespace().collect();
        let dist = data.pop().unwrap().parse::<i32>()
            .expect("Error parsing distance");
        let dir = data.pop().unwrap();
        let direction = match dir {
            "forward" => Directions::Forward(dist),
            "down" => Directions::Down(dist),
            "up" => Directions::Down(-dist),
            _ => Directions::Forward(0)
        };
        direction
    }
}