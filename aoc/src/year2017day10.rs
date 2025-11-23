use std::{fmt::Display, io::Read, process::id};

use itertools::Itertools;

pub fn parse(input: &str) -> Vec<u8> {
    input
        .split(',')
        .map(|num| num.parse::<u8>().unwrap())
        .collect()
}

pub fn part1(lengths: &Vec<u8>) -> impl Display {
    let twisted_rope = knot_rope((0..=255).collect(), lengths);
    twisted_rope
        .into_iter()
        .take(2)
        .fold(1u64, |acc, x| acc * x as u64)
}

pub fn part2(lengths: &Vec<u8>) -> impl Display {
    0
}

fn knot_rope(rope: Vec<u8>, knot_lengths: &Vec<u8>) -> Vec<u8> {
    let rope_length: usize = rope.len();
    let mut current_position = 0;
    let mut skip_size = 0;
    let mut twisted_rope: Vec<u8> = rope.clone();
    for length in knot_lengths {
        let infinite_rope = twisted_rope.iter().cycle();
        let rope_slice = infinite_rope
            .skip(current_position)
            .take(*length as usize)
            .map(|x| x.clone())
            .collect::<Vec<u8>>();
        let reversed_rope_section = rope_slice.iter().rev();

        for x in reversed_rope_section {
            twisted_rope[current_position] = *x;
            current_position = (current_position + 1) % rope_length;
        }
        current_position = (current_position + skip_size) % rope_length;
        skip_size += 1;
    }
    twisted_rope
}

fn condense_hash(sparse_hash: &Vec<u8>) -> Vec<u8> {
    sparse_hash
        .chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, x| acc ^ x))
        .collect()
}

fn print_dense_hash(dense_hash: &Vec<u8>) -> String {
    dense_hash.iter().map(|x| format!("{:02x}", x)).collect()
}

#[cfg(test)]
mod tests {
    use crate::year2017day10::{condense_hash, knot_rope, print_dense_hash};

    #[test]
    fn test_knot_rope() {
        let result = knot_rope((0..5).collect(), &vec![3, 4, 1, 5]);
        assert_eq!(result, vec![3, 4, 2, 1, 0])
    }

    #[test]
    fn test_condense_hash() {
        let result = condense_hash(&vec![65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 64);
    }

    #[test]
    fn test_print_dense_hash() {
        let result = print_dense_hash(&vec![64, 7, 255]);
        assert_eq!(result, "4007ff");
    }
}
