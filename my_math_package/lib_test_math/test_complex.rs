#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let c1 = Complex::new(2.0, 3.0);
        let c2 = Complex::new(4.0, 5.0);
        let result = c1.add(c2);
        assert_eq!(result.real, 6.0);
        assert_eq!(result.imaginary, 8.0);
    }

    #[test]
    fn test_subtract() {
        let c1 = Complex::new(6.0, 8.0);
        let c2 = Complex::new(4.0, 5.0);
        let result = c1.subtract(c2);
        assert_eq!(result.real, 2.0);
        assert_eq!(result.imaginary, 3.0);
    }

    #[test]
    fn test_multiply() {
        let c1 = Complex::new(2.0, 3.0);
        let c2 = Complex::new(4.0, 5.0);
        let result = c1.multiply(c2);
        assert_eq!(result.real, -7.0);
        assert_eq!(result.imaginary, 22.0);
    }

    #[test]
    fn test_divide() {
        let c1 = Complex::new(2.0, 3.0);
        let c2 = Complex::new(4.0, 5.0);
        let result = c1.divide(c2);
        assert_eq!(result.real, 0.5609756097560976);
        assert_eq!(result.imaginary, 0.0487804878048781);
    }
}
