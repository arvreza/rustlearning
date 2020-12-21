#[allow(dead_code)]

fn fib1(n : i32) -> i32 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }

    return fib1(n - 1) + fib1(n - 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_or_one() {
        assert_eq!(fib1(0), 0);
        assert_eq!(fib1(1), 1);
    }

    #[test]
    fn test_6() {
        assert_eq!(fib1(6), 8);
    }
}
