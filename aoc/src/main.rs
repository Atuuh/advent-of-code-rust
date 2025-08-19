use std::{fs, path::Path, time::Instant};

#[derive(Debug)]
pub enum AocError {
    Allsorts,
}

pub mod year2017day01;
pub mod year2017day02;
pub mod year2017day03;
pub mod year2017day04;
pub mod year2017day05;
pub mod year2017day06;
pub mod year2017day07;
pub mod year2017day08;

fn main() {
    let year = 2017;
    let day = 8;
    let input_result = get_input(year, day).unwrap();

    use crate::year2017day08::{parse, part1, part2};
    let initial_time = Instant::now();
    let parsed_input = parse(input_result.as_str());
    let after_parsed_input = Instant::now();
    let part1_answer = part1(&parsed_input);
    let after_part1 = Instant::now();
    let part2_answer = part2(&parsed_input);
    let after_part2 = Instant::now();

    println!(
        "Part 1: {part1_answer} ({:?}), Part 2: {part2_answer} ({:?})",
        after_part1.duration_since(after_parsed_input),
        after_part2.duration_since(after_part1)
    )
}

fn get_input(year: u16, day: u8) -> Result<String, AocError> {
    let path_string = format!("./inputs/{year}/{day:0>2}.txt");
    let path = Path::new(&path_string);
    let result = fs::read_to_string(path).map(|s| s.trim().to_owned());
    result.map_err(|_e| AocError::Allsorts)
}
