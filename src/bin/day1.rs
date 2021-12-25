use advent_of_code_2021 as aoc;

fn main() {
    let filepath = aoc::get_filepath(1);
    if filepath.exists() {
        let path_str = filepath.to_str().expect("Error getting path");
        let file_str = aoc::read_file_to_string(path_str);
        let numbers = aoc::read_string_separate_lines(&file_str);
        let number_vec: Vec<i32> = numbers
            .into_iter()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();
        let counter = single_changes(&number_vec);
        let counter_windows = sliding_window(&number_vec, 3);
        println!("There are {} positive changes in depth.", counter);
        println!(
            "There are {} positive sliding window changes in depth",
            counter_windows
        );
    } else {
        println!("File does not exist!");
    }
}

fn single_changes(vec: &Vec<i32>) -> i32 {
    let mut iter = vec.iter();
    let mut prev = *iter.next().expect("Empty vector!");
    let mut counter = 0;
    for item in iter {
        if *item > prev {
            counter += 1;
        }
        prev = *item;
    }
    counter
}

fn sliding_window(vec: &Vec<i32>, window_size: usize) -> i32 {
    let new_vec = vec.clone();
    let mut windows = new_vec.windows(window_size).peekable();
    let mut prev = windows.next().expect("Empty vector!");
    let mut counter = 0;
    for item in windows {
        if item.into_iter().sum::<i32>() > prev.into_iter().sum() {
            counter += 1;
        }
        prev = item.clone();
    }
    counter
}
