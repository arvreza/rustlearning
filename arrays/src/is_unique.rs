#[allow(dead_code)]
use std::collections::HashMap;

fn is_unique(_value: &str) -> bool {
    let mut _checker: HashMap<char, i32> = HashMap::new();

    for c in _value.chars() {
        if _checker.contains_key(&c) {
            return false;
        } else {
            _checker.insert(c, 0);
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unique_false() {
        let test = "RezaArbabi";
        assert_eq!(is_unique(test), false);
    }

    #[test]
    fn test_is_unique_true() {
        let test = "RezaAr";
        assert_eq!(is_unique(test), true);
    }
}
