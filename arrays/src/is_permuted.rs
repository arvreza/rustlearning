#[allow(dead_code)]
use std::collections::HashMap;

fn is_permuted(val1: &str, val2: &str) -> bool {
    let mut _checker: HashMap<char, i32> = HashMap::new();

    //populate the set
    for c in val1.chars() {
        let counter = _checker.entry(c).or_insert(0);
        *counter += 1;
    }

    for c in val2.chars() {
        let counter = _checker.get_mut(&c);

        if counter == None  {
            return false;
        }

        *counter.unwrap() -= 1;
    }

    let sum: i32 = _checker.values().sum();
    if sum == 0 {
        return true;
    }
        
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permuted() {
        let val1 = "reza";
        let val2 = "arez";

        assert_eq!(is_permuted(&val1, &val2), true);
    }

    #[test]
    fn test_not_permuted() {
        let val1 = "reza";
        let val2 = "arez1";
        
        assert_eq!(is_permuted(&val1, &val2), false);
    }

    #[test]
    fn test_not_permuted2() {
        let val1 = "reza1";
        let val2 = "arez";
        
        assert_eq!(is_permuted(&val1, &val2), false);
    }

    #[test]
    fn test_not_permuted3() {
        let val1 = "aa";
        let val2 = "aaa";
        
        assert_eq!(is_permuted(&val1, &val2), false);
    }

}