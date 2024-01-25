// Watch https://www.youtube.com/watch?v=_JtPhF8MshA

use tailcall::tailcall;

pub fn factorial_tail_recursion(n: u128) -> u128 {
    #[tailcall]
    fn factorial_inner(accumulator: u128, n: u128) -> u128 {
        match n {
            0 => accumulator,
            _ => factorial_inner(accumulator * n, n - 1),
        }
    }
    factorial_inner(1, n)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial_tail_recursion(0), 1);
        assert_eq!(factorial_tail_recursion(1), 1);
        assert_eq!(factorial_tail_recursion(10), 3628800);
    }
}
