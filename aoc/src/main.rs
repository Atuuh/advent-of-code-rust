use std::{fs, path::Path};

use crate::year2017day01::{solve};

pub enum AocError {
    Allsorts
}

pub mod year2017day01;

fn main() {
    let year = 2017;
    let day = 1;
    let input_result= get_input(year, day);

    let result = input_result.and_then(solve);
    match result {
        Ok(value) => {
            println!("Answer is {value}")
        },
        Err(_) => {
            println!("Failed to work out answer")
        },
    }
}

fn get_input(year: u16, day: u8) -> Result<String, AocError> {
    let path_string = format!("./inputs/{year}/day-{day:0>2}.txt");
    let path = Path::new(&path_string);
    let result = fs::read_to_string(path).map(|s | s.trim().to_owned());
    result.map_err(|_e | AocError::Allsorts)
}