use std::ops::{Add, Mul};

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

            Ok(result.to_string())
        }
    }
}

fn get_layer_number(value: u32) -> u32 {
    let sqrt = (value - 1).isqrt();

    if sqrt % 2 == 0 {
        sqrt / 2
    } else {
        (sqrt + 1) / 2
    }
}

fn _get_side(value: u32, layer: u32) -> u32 {
    let x = value - ((layer + 1) ^ 2);
    x / (layer * 2) - 1
}

fn get_offset(value: u32, layer: u32) -> u32 {
    let adjusted_value = i32::try_from(value - get_layer_max_value(layer - 1)).unwrap();
    let side_length = i32::try_from(layer * 2).unwrap();
    let remainder = adjusted_value % side_length;
    layer - (remainder - (side_length / 2)).unsigned_abs()
}

fn get_layer_max_value(layer: u32) -> u32 {
    layer.mul(2).add(1).pow(2)
}
