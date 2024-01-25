pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 2) + fibonacci(n - 1),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
        assert_eq!(fibonacci(15), 610);
        assert_eq!(fibonacci(40), 102334155);
    }
}
