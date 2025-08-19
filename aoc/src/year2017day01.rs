use crate::AocError;

pub fn solve(input: String) -> Result<String, AocError> {
    let numbers: Vec<u32> = input.chars().filter_map(|char| char.to_digit(10)).collect();
    let offset = numbers.len() / 2;
    let zipped = zip_self_with_offset(&numbers, offset);
    let result = zipped.fold(0, |a, (x, y)| if x == y { a + x } else { a });
    Ok(result.to_string())
}

fn zip_self_with_offset<T>(xs: &[T], offset: usize) -> impl Iterator<Item = (&T, &T)> {
    let len = xs.len();
    xs.iter()
        .enumerate()
        .map(move |(i, x)| (x, &xs[(i + offset) % len]))
}
