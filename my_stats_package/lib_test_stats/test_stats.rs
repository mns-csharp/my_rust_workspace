#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistics() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let statistics = Statistics::new(data.clone());

        assert_eq!(statistics.mean(), 3.0);
        assert_eq!(statistics.median(), 3.0);
        assert_eq!(statistics.mode(), None);

        assert_eq!(statistics.variance().unwrap(), 2.5);
        assert_eq!(statistics.standard_deviation().unwrap(), 1.5811388300841898);

        let data = vec![1.0, 2.0, 3.0, 4.0, 4.0, 5.0];
        let statistics = Statistics::new(data.clone());

        assert_eq!(statistics.mean(), 3.1666666666666665);
        assert_eq!(statistics.median(), 3.5);
        assert_eq!(statistics.mode(), Some(4.0));

        assert_eq!(statistics.variance().unwrap(), 1.4722222222222223);
        assert_eq!(statistics.standard_deviation().unwrap(), 1.2154378355006818);
    }
}
