#[allow(dead_code)]
use std::char;
use std::collections::HashMap;

fn find_non_repeating(value: &str) -> char {

    let mut letters = HashMap::new();

    for c in value.chars() {
        *letters.entry(c).or_insert(0) += 1;
    }

    for c in value.chars() {
        if letters.get(&c).unwrap() == &1 {
            return c;
        }
    }

    return char::from_digit(0, 10).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_non_repeating() {
        assert_eq!(find_non_repeating("total"), 'o');
        assert_eq!(find_non_repeating("teeter"), 'r');
    }
}