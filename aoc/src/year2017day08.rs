use std::{
    collections::HashMap,
    fmt::Display,
};

pub struct Instruction {
    check_register: String,
    operator: String,
    check_value: i32,
    success_register: String,
    success_value: i32,
}

pub fn parse(input: &str) -> Vec<Instruction> {
    input
        .split("\n")
        .map(|line| {
            let vvvv: Vec<_> = line.split(" ").collect();
            match vvvv[..] {
                [a, b, c, _if, d, e, f] => Instruction {
                    check_register: d.to_string(),
                    operator: e.to_string(),
                    check_value: f.parse().unwrap(),
                    success_register: a.to_string(),
                    success_value: match b {
                        "inc" => c.parse().unwrap(),
                        "dec" => c.parse::<i32>().unwrap() * -1,
                        _ => panic!("invalid operator"),
                    },
                },
                _ => panic!("invalid input"),
            }
        })
        .collect()
}

pub fn part1(instructions: &Vec<Instruction>) -> impl Display {
    let mut registers: HashMap<_, _> = HashMap::new();

    for instruction in instructions {
        let value = *registers
            .get(&instruction.check_register.clone())
            .unwrap_or(&0);
        let succeeds = match instruction.operator.as_str() {
            ">" => value > instruction.check_value,
            "<" => value < instruction.check_value,
            ">=" => value >= instruction.check_value,
            "<=" => value <= instruction.check_value,
            "==" => value == instruction.check_value,
            "!=" => value != instruction.check_value,
            operator => panic!("invalid instruction {operator}"),
        };

        if succeeds {
            let v = registers
                .entry(instruction.success_register.clone())
                .or_insert(0);
            *v += instruction.success_value;
        }
    }

    let result = registers.values().max().unwrap_or(&0).clone();
    result
}

pub fn part2(instructions: &Vec<Instruction>) -> impl Display {
    let mut registers: HashMap<_, _> = HashMap::new();
    let mut max_value = 0;

    for instruction in instructions {
        let value = *registers
            .get(&instruction.check_register.clone())
            .unwrap_or(&0);
        let succeeds = match instruction.operator.as_str() {
            ">" => value > instruction.check_value,
            "<" => value < instruction.check_value,
            ">=" => value >= instruction.check_value,
            "<=" => value <= instruction.check_value,
            "==" => value == instruction.check_value,
            "!=" => value != instruction.check_value,
            operator => panic!("invalid instruction {operator}"),
        };

        if succeeds {
            let v = registers
                .entry(instruction.success_register.clone())
                .or_insert(0);
            *v += instruction.success_value;

            if *v > max_value {
                max_value = *v;
            }
        }
    }

    max_value
}

#[cfg(test)]
mod tests {}
