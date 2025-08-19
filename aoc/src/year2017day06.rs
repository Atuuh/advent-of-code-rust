use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub fn parse(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|x| x.parse::<u32>())
        .flatten()
        .collect()
}

pub fn part1(numbers: &Vec<u32>) -> impl Display {
    let mut memory = numbers.clone();
    let mut visited = HashSet::new();
    let mut index = 0;
    while !visited.contains(&memory) {
        visited.insert(memory.clone());
        redistribute_blocks(&mut memory);
        index += 1;
    }
    index
}

pub fn part2(numbers: &Vec<u32>) -> impl Display {
    let mut memory = numbers.clone();
    let mut visited = HashMap::new();
    let mut index = 0;
    while !visited.contains_key(&memory) {
        visited.insert(memory.clone(), index);
        redistribute_blocks(&mut memory);
        index += 1;
    }
    index - visited[&memory]
}

fn redistribute_blocks(memory_banks: &mut Vec<u32>) {
    let length = memory_banks.len();
    let (index, max_value) = memory_banks
        .iter()
        .enumerate()
        .rev()
        .max_by(|(_ix, x), (_iy, y)| x.cmp(y))
        .unwrap()
        .clone();

    let m_index = index.clone();
    let m_max_value = max_value.clone();
    memory_banks[m_index] = 0;

    for x in 1..m_max_value + 1 {
        let target_index = (m_index + x as usize) % length;
        memory_banks[(target_index) % length] += 1
    }
}

#[cfg(test)]
mod tests {
    use crate::year2017day06::{part2, redistribute_blocks};

    #[test]
    fn test_redistribute_blocks() {
        let mut memory = vec![0, 2, 7, 0];
        redistribute_blocks(&mut memory);
        assert_eq!(memory, vec![2, 4, 1, 2]);
        redistribute_blocks(&mut memory);
        assert_eq!(memory, vec![3, 1, 2, 3]);
        redistribute_blocks(&mut memory);
        assert_eq!(memory, vec![0, 2, 3, 4]);
        redistribute_blocks(&mut memory);
        assert_eq!(memory, vec![1, 3, 4, 1]);
    }

    #[test]
    fn test_part2() {
        let memory = vec![0, 2, 7, 0];
        let result = part2(&memory);
        assert_eq!(result.to_string(), 4.to_string())
    }
}
