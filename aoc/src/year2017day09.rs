use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Group {
    depth: u16,
    garbage: u16,
}

impl Group {
    fn new(depth: u16) -> Group {
        Group {
            depth: depth,
            garbage: 0,
        }
    }

    fn add_garbage(&mut self) -> () {
        self.garbage += 1;
    }
}

pub fn parse(input: &str) -> Vec<Group> {
    let mut groups: Vec<Group> = vec![];
    let mut current_group: &mut Group = &mut Group::new(0);
    let mut should_ignore: bool = false;
    let mut is_in_garbage: bool = false;
    let mut current_depth: u16 = 0;

    for ch in input.chars() {
        match (ch, should_ignore, is_in_garbage) {
            (_, true, _) => should_ignore = false,
            ('!', false, _) => should_ignore = true,
            ('>', false, true) => is_in_garbage = false,
            ('<', false, false) => is_in_garbage = true,
            ('{', false, false) => {
                current_depth += 1;
                // let mut new_group = Group::new(current_depth);
                let new_group = Group::new(current_depth);
                groups.push(new_group);
                current_group = groups.last_mut().unwrap()
            }
            ('}', false, false) => {
                current_depth -= 1;
            }
            (_, false, true) => current_group.add_garbage(),
            _ => continue,
        }
    }
    groups
}

pub fn part1(groups: &Vec<Group>) -> impl Display {
    groups.iter().fold(0, |acc, group| acc + group.depth)
}

pub fn part2(groups: &Vec<Group>) -> impl Display {
    groups.iter().fold(0, |acc, group| acc + group.garbage)
}

#[cfg(test)]
mod tests {
    // use crate::year2017day09

    use crate::year2017day09::{parse, part1, part2};

    #[test]
    fn test_group_counts() {
        assert_eq!(parse("{}").len(), 1);
        assert_eq!(parse("{{{}}}").len(), 3);
        assert_eq!(parse("{{},{}}").len(), 3);
        assert_eq!(parse("{{{},{},{{}}}}").len(), 6);
        assert_eq!(parse("{<{},{},{{}}>}").len(), 1);
        assert_eq!(parse("{<a>,<a>,<a>,<a>}").len(), 1);
        assert_eq!(parse("{{<a>},{<a>},{<a>},{<a>}}").len(), 5);
        assert_eq!(parse("{{<!>},{<!>},{<!>},{<a>}}").len(), 2);
    }

    #[test]
    fn test_group_scores() {
        assert_eq!(part1(&parse("{}")).to_string(), 1.to_string());
        assert_eq!(part1(&parse("{{{}}}")).to_string(), 6.to_string());
        assert_eq!(part1(&parse("{{},{}}")).to_string(), 5.to_string());
        assert_eq!(part1(&parse("{{{},{},{{}}}}")).to_string(), 16.to_string());
        assert_eq!(
            part1(&parse("{<a>,<a>,<a>,<a>}")).to_string(),
            1.to_string()
        );
        assert_eq!(
            part1(&parse("{{<ab>},{<ab>},{<ab>},{<ab>}}")).to_string(),
            9.to_string()
        );
        assert_eq!(
            part1(&parse("{{<!!>},{<!!>},{<!!>},{<!!>}}")).to_string(),
            9.to_string()
        );
        assert_eq!(
            part1(&parse("{{<a!>},{<a!>},{<a!>},{<ab>}}")).to_string(),
            3.to_string()
        );
    }

    #[test]
    fn test_garbage_sizes() {
        let input = vec![
            ("{<>}", 0),
            ("{<random characters>}", 17),
            ("{<<<<>}", 3),
            ("{<{!>}>}", 2),
            ("{<!!>}", 0),
            ("{<!!!>>}", 0),
            (r#"{<{o"i!a,<{i<a>}"#, 10),
        ];

        for (text, expected_garbage_count) in input {
            let groups = parse(text);
            let result = part2(&groups);
            assert_eq!(result.to_string(), expected_garbage_count.to_string())
        }
    }
}
