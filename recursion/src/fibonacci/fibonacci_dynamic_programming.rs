use crate::fibonacci::fibonacci::fibonacci;

pub fn fibonacci_dynamic_programming(n: u64) -> u64 {
    let mut memoized_values: Vec<u64> = vec![0, 1];
    return fibonacci_on_the_fly(&mut memoized_values, n);

    fn fibonacci_on_the_fly(memoized_values: &mut Vec<u64>, n: u64) -> u64 {
        let n_as_usize: usize = n as usize;
        if n_as_usize >= memoized_values.len() {
            let fib_minus_1 = fibonacci_on_the_fly(memoized_values, n - 1);
            let fib_minus_2 = fibonacci_on_the_fly(memoized_values, n - 2);
            memoized_values.push(fib_minus_1 + fib_minus_2);
        }
        memoized_values[n_as_usize]
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
