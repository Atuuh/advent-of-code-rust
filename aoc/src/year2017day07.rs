use regex::Regex;
use std::{collections::HashMap, fmt::Display};

#[derive(Default, Clone)]
struct Disc {
    name: String,
    weight: u32,
}

struct DiscWithChildren {
    disc: Disc,
    children: Vec<String>,
}

enum FDisc {
    NoChildren(Disc),
    Children(DiscWithChildren),
}

pub fn parse(input: &str) {
    let re = Regex::new(r"(?<name>\w+) \((?<weight>\w+)\)(?: -> (?<children>.*))?").unwrap();

    let parsed = input.split("\n").map(|line| {
        let captures = re.captures(line).unwrap();
        let name = captures["names"].to_string();
        let weight: u32 = captures["weight"].parse().unwrap();
        let children_string = captures.name("children");

        Disc { name, weight }
    });

    let names_and_data: Vec<_> = input
        .split("\n")
        .map(|line| line.split_once("").unwrap())
        .collect();
    let disc_indices: HashMap<_, _> = names_and_data
        .iter()
        .enumerate()
        .map(|(i, &(k, _))| (k, i))
        .collect();
    let discs = vec![Disc::default(); disc_indices.len()];
}

pub fn part1(numbers: &Vec<u32>) -> impl Display {
    0
}

pub fn part2(numbers: &Vec<u32>) -> impl Display {
    0
}

#[cfg(test)]
mod tests {}
