use std::fs;
use std::vec::Vec;
//use std::string::String;

pub fn read_file_to_string(path: &str) -> String {
    fs::read_to_string(path)
        .expect("Something went wrong when reading the file")
}

pub fn read_string_separate_lines(whole_string: &str) -> Vec<&str> {
    whole_string.lines().collect()
}