use itertools::Itertools;

use crate::AocError;

pub fn solve(input: String) -> Result<String, AocError> {
    let result = input
        .split('\n')
        .map(is_passphrase_super_valid)
        .filter(|x| *x)
        .count();

    Ok(result.to_string())
}

fn is_passphrase_valid(input: &str) -> bool {
    let words: Vec<&str> = input.split_whitespace().collect();
    words.len() == words.iter().unique().count()
}

fn is_passphrase_super_valid(input: &str) -> bool {
    let words: Vec<&str> = input.split_whitespace().collect();
    let sorted_words: Vec<String> = words
        .iter()
        .map(|word| word.chars().sorted().collect())
        .collect();
    sorted_words.len() == sorted_words.iter().unique().count()
}

#[cfg(test)]
mod tests {
    use crate::year2017day04::is_passphrase_super_valid;

    #[test]
    fn test_is_password_super_valid() {
        assert_eq!(is_passphrase_super_valid("abcde fghij"), true);
        assert_eq!(is_passphrase_super_valid("abcde xyz ecdab"), false);
        assert_eq!(is_passphrase_super_valid("a ab abc abd abf abj"), true);
        assert_eq!(is_passphrase_super_valid("iiii oiii ooii oooi oooo"), true);
        assert_eq!(is_passphrase_super_valid("oiii ioii iioi iiio"), false);
    }
}
