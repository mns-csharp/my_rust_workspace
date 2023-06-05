#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr1;
    use ndarray::Array;

    #[test]
    fn test_classifier() {
        let num_features = 2;
        let mut classifier = Classifier::new(num_features);

        let features = Array::from_shape_vec((3, num_features), vec![1.0, 1.0, 2.0, 2.0, 3.0, 3.0]).unwrap();
        let labels = arr1(&[0.0, 0.0, 1.0]);

        let num_iterations = 100;
        let learning_rate = 0.1;

        classifier.train(&features, &labels, num_iterations, learning_rate);

        let test_features = Array::from_shape_vec((2, num_features), vec![4.0, 4.0, 5.0, 5.0]).unwrap();
        let predictions = classifier.predict(&test_features);

        assert_eq!(predictions, arr1(&[1.0, 1.0]));
    }
}
