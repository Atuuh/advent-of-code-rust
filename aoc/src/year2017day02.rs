use crate::AocError;
use itertools::Itertools;

pub fn solve(input: String) -> Result<String, AocError> {
    let data: Vec<Vec<u32>> = input
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.trim().parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let line_differences = data.iter().map(evenly_divisible);
    let result: u32 = line_differences.flatten().sum();
    Ok(result.to_string())
}

fn get_min_max_distance(line: &Vec<u32>) -> Option<u32> {
    let minmax = line.iter().minmax().into_option();
    let distance = minmax.map(|(min, max)| max - min);
    distance
}

// TODO: This is O(n^2), please fix
fn evenly_divisible(line: &Vec<u32>) -> Option<u32> {
    line.iter().find_map(|x| {
        line.iter().find_map(|y| {
            if x == y {
                None
            } else if x.is_multiple_of(*y) {
                Some(x / y)
            } else if y.is_multiple_of(*x) {
                Some(y / x)
            } else {
                None
            }
        })
    })
}
