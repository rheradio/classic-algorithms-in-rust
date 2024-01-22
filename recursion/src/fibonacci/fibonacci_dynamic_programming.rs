use crate::fibonacci::fibonacci::fibonacci;

pub fn fibonacci_dynamic_programming(n: u64) -> u64 {
    if n <= 1 {
        return n;
    } else {
        let mut memoized_values: Vec<u64> = vec!(0, 1);
        return fibonacci_on_the_fly(&mut memoized_values, n);
    }
    fn fibonacci_on_the_fly(memoized_values: &mut Vec<u64>, n: u64) -> u64 {
        let last_computed = memoized_values.len() - 1;
        let n1: usize = (n - 1) as usize;
        let n2: usize = (n - 2) as usize;
        let result: u64;
        if last_computed == n1 {
            result = memoized_values[n1] + memoized_values[n2];
        } else {
            result = fibonacci_on_the_fly(memoized_values, n - 1) + memoized_values[n2];
        }
        memoized_values.push(result);
        result
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fibonacci_dynamic_programming() {
        assert_eq!(fibonacci_dynamic_programming(0), 0);
        assert_eq!(fibonacci_dynamic_programming(1), 1);
        assert_eq!(fibonacci_dynamic_programming(10), 55);
        assert_eq!(fibonacci_dynamic_programming(15), 610);
        assert_eq!(fibonacci_dynamic_programming(40), 102334155);
    }
}

