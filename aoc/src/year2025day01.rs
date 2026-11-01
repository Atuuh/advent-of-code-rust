use std::fmt::Display;

pub fn parse(input: &str) -> Vec<i32> {
    input
        .split('\n')
        .map(|num| {
            let (x, y) = num.split_at(1);
            let dir = match x {
                "L" => -1,
                "R" => 1,
                _ => panic!("invalid input: {}", num),
            };
            let number = y.parse::<i32>().unwrap();
            dir * number
        })
        .collect()
}

pub fn part1(turns: &Vec<i32>) -> impl Display {
    let mut dial = Dial::new();
    let mut count = 0;
    for turn in turns {
        dial.rotate(turn);
        if dial.position == 0 {
            count += 1;
        }
    }
    count
}

pub fn part2(turns: &Vec<i32>) -> impl Display {
    let mut dial = Dial::new();
    let mut count = 0;
    for turn in turns {
        let x: i32 = turn + i32::try_from(dial.position).unwrap();
        let y = x.abs().div_euclid(100);
        if dial.position != 0 && x < 0 {
            count += 1;
        }
        count += y;
        dial.rotate(turn);
        if *turn < 0 && dial.position == 0 {
            count += 1;
        }
    }

    count
}

#[derive(Debug)]
struct Dial {
    position: u32,
}

impl Dial {
    fn new() -> Self {
        Self { position: 50 }
    }
    fn rotate(&mut self, amount: &i32) {
        let position: i32 = self.position.try_into().unwrap();
        let new_position = (position + amount).rem_euclid(100);
        self.position = new_position.try_into().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::year2025day01::{Dial, part2};

    #[test]
    fn test() {
        let mut dial = Dial::new();
        assert_eq!(dial.position, 50);

        dial.rotate(&-68);
        assert_eq!(dial.position, 82);

        dial.rotate(&-30);
        assert_eq!(dial.position, 52);

        dial.rotate(&48);
        assert_eq!(dial.position, 0);

        dial.rotate(&-5);
        assert_eq!(dial.position, 95);

        dial.rotate(&60);
        assert_eq!(dial.position, 55);

        dial.rotate(&-55);
        assert_eq!(dial.position, 0);

        dial.rotate(&-1);
        assert_eq!(dial.position, 99);

        dial.rotate(&-99);
        assert_eq!(dial.position, 0);

        dial.rotate(&14);
        assert_eq!(dial.position, 14);

        dial.rotate(&-82);
        assert_eq!(dial.position, 32);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part2(&vec![1000]).to_string(), 10.to_string());
        assert_eq!(
            part2(&vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]).to_string(),
            6.to_string()
        );
    }
}
