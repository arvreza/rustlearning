#[allow(dead_code)]
fn sum(_slice: &[i32]) -> i32 {

    let mut _sum = 0;
    for i in _slice.iter() {
        _sum += i;
    }
    return _sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let test1:[i32; 5] = [1, 2, 3, 4, 5];

        assert_eq!(sum(&test1), 15);
    }

    #[test]
    fn test_sum0() {
        let test1:[i32; 1] = [0];

        assert_eq!(sum(&test1), 0);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_bigsum() {
        let test1:[i32; 2] = [i32::MAX, 1];

        assert_eq!(sum(&test1), 0);
    }
}

