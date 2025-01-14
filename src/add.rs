pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
    #[test]
    fn test_add_multiply_ambiguous() {
        assert_eq!(add(2, 2), 4);
    }
    #[test]
    fn test_add_zero() {
        assert_eq!(add(2, 0), 2);
    }
}
