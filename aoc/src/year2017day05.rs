use std::ops::Index;

use crate::AocError;

pub fn solve(input: String) -> Result<String, AocError> {
    let instructions = input.split('\n').map(|x| x.parse().unwrap()).collect();
    let result = steps_before_exits(instructions, conditional_decrement);
    Ok(result.to_string())
}

fn steps_before_exits(moves: Vec<i32>, modify_offset_fn: fn(i32) -> i32) -> u32 {
    let mut index: i32 = 0;
    let mut jumps = 0;
    let mut instructions = moves.clone();
    loop {
        let index_usize = usize::try_from(index).unwrap();
        let instruction = instructions.get(index_usize);
        match instruction {
            Some(x) => {
                index += x;
                instructions[index_usize] += modify_offset_fn(*x);
                jumps += 1;
            }
            None => return jumps,
        }
    }
}

fn always_increment(offset: i32) -> i32 {
    1
}

fn conditional_decrement(offset: i32) -> i32 {
    if offset >= 3 { -1 } else { 1 }
}

#[cfg(test)]
mod tests {
    use crate::year2017day05::steps_before_exits;

    #[test]
    fn test_steps_before_exits() {
        assert_eq!(steps_before_exits(vec![0, 3, 0, 1, -3]), 5)
    }
}
