#[allow(dead_code)]
fn max(_slice:&[i32]) -> &i32 {
    let mut _max = &i32::MIN;
    
    for n in _slice.iter() {
        if n > _max {
            _max = n;
        }
    }

    return _max;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        let test_array = [4, 10, 35, 9];

        assert_eq!(max(&test_array), &35);
    }
}