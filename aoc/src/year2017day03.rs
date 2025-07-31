use std::{
    collections::HashMap,
    ops::{Add, Mul, Neg, Sub},
};

use crate::AocError;

pub fn solve(input: String) -> Result<String, AocError> {
    let value = input.trim().parse::<u32>();

    match value {
        Ok(1) => Ok(0.to_string()),
        Err(_) => Err(AocError::Allsorts),
        Ok(value) => {
            let layer = get_layer_number(value);

            let offset = get_offset(value, layer);

            let result = layer + layer - offset;

            Ok(part_two(value).to_string())
        }
    }
}

fn part_two(input: u32) -> u32 {
    let mut result = 1;
    let mut position_values: HashMap<Position, u32> = HashMap::from([(Position::new(0, 0), 1)]);
    let mut index = 1;
    let directions = vec![
        Position::UP,
        Position::UP + Position::RIGHT,
        Position::RIGHT,
        Position::RIGHT + Position::DOWN,
        Position::DOWN,
        Position::DOWN + Position::LEFT,
        Position::LEFT,
        Position::LEFT + Position::UP,
    ];
    while result < input {
        index += 1;
        let position = get_position_from_index(index);
        result = directions
            .iter()
            .map(|dir| position_values.get(&position.add(*dir)))
            .flatten()
            .sum();
        position_values.insert(position, result);
    }
    result
}

#[derive(PartialEq, Debug, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Eq for Position {}
impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Neg for Position {
    type Output = Position;

    fn neg(self) -> Self::Output {
        Position::new(-self.x, -self.y)
    }
}
impl Position {
    const UP: Position = Position::new(0, 1);
    const DOWN: Position = Position::new(0, -1);
    const LEFT: Position = Position::new(-1, 0);
    const RIGHT: Position = Position::new(1, 0);

    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn is_adjacent(self: Self, other: Self) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }
}

fn get_position_from_index(index: u32) -> Position {
    match index {
        1 => Position::new(0, 0),
        _ => {
            let layer = get_layer_number(index);
            let ilayer = i32::try_from(layer).unwrap();
            let side = _get_side(index, layer);
            let offset: i32 = get_offset(index, layer).try_into().unwrap();
            let (x, y) = match side {
                0 => (ilayer, ilayer - offset),
                1 => (-ilayer + offset, ilayer),
                2 => (-ilayer, -ilayer + offset),
                3 => (ilayer - offset, -ilayer),
                _ => (0, 0),
            };
            // println!("index {index} layer {layer} side {side} offset {offset} (x,y) {x},{y}");
            return Position::new(x, y);
        }
    }
}

fn get_layer_number(value: u32) -> u32 {
    match value {
        1 => 0,
        _ => {
            let sqrt = (value - 1).isqrt();

            if sqrt % 2 == 0 {
                sqrt / 2
            } else {
                (sqrt + 1) / 2
            }
        }
    }
}

fn _get_side(value: u32, layer: u32) -> u32 {
    match layer {
        0 => 0,
        _ => {
            let adjusted_value = value - get_layer_max_value(layer - 1) - 1;
            // println!(
            //     "{value} adjusted to {adjusted_value} = {}",
            //     adjusted_value / (layer * 2)
            // );
            adjusted_value / (layer * 2)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::year2017day03::{
        _get_side, Position, get_layer_number, get_offset, get_position_from_index,
    };

    #[test]
    fn test_get_layer() {
        assert_eq!(get_layer_number(1), 0);
        assert_eq!(get_layer_number(2), 1);
        assert_eq!(get_layer_number(3), 1);
        assert_eq!(get_layer_number(4), 1);
        assert_eq!(get_layer_number(5), 1);
        assert_eq!(get_layer_number(6), 1);
        assert_eq!(get_layer_number(7), 1);
        assert_eq!(get_layer_number(8), 1);
        assert_eq!(get_layer_number(9), 1);
        assert_eq!(get_layer_number(10), 2);
        assert_eq!(get_layer_number(11), 2);
    }

    #[test]
    fn test_get_position_from_index() {
        assert_eq!(get_position_from_index(1), Position::new(0, 0));
        assert_eq!(get_position_from_index(2), Position::new(1, 0));
        assert_eq!(get_position_from_index(3), Position::new(1, 1));
        assert_eq!(get_position_from_index(4), Position::new(0, 1));
        assert_eq!(get_position_from_index(5), Position::new(-1, 1));
        assert_eq!(get_position_from_index(6), Position::new(-1, 0));
        assert_eq!(get_position_from_index(7), Position::new(-1, -1));
        assert_eq!(get_position_from_index(8), Position::new(0, -1));
        assert_eq!(get_position_from_index(9), Position::new(1, -1));
    }

    #[test]
    fn test_get_offset() {
        assert_eq!(get_offset(2, 1), 1);
        assert_eq!(get_offset(3, 1), 0);
        assert_eq!(get_offset(4, 1), 1);
        assert_eq!(get_offset(5, 1), 0);
        assert_eq!(get_offset(6, 1), 1);
        assert_eq!(get_offset(7, 1), 0);
        assert_eq!(get_offset(8, 1), 1);
        assert_eq!(get_offset(9, 1), 0);
    }

    #[test]
    fn test_get_side() {
        assert_eq!(_get_side(2, 1), 0);
        assert_eq!(_get_side(3, 1), 0);
        assert_eq!(_get_side(4, 1), 1);
        assert_eq!(_get_side(5, 1), 1);
        assert_eq!(_get_side(6, 1), 2);
        assert_eq!(_get_side(7, 1), 2);
        assert_eq!(_get_side(8, 1), 3);
        assert_eq!(_get_side(9, 1), 3);
    }
}

fn get_offset(value: u32, layer: u32) -> u32 {
    let adjusted_value = i32::try_from(value - get_layer_max_value(layer - 1) - 1).unwrap();
    let side_length = i32::try_from(layer * 2).unwrap();
    let remainder = adjusted_value % side_length;
    // layer - (remainder - (side_length / 2)).unsigned_abs()
    let offset = side_length - remainder - 1;
    offset.try_into().unwrap()
}

fn get_layer_max_value(layer: u32) -> u32 {
    layer.mul(2).add(1).pow(2)
}
