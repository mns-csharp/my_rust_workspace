fn main() {
    // Generate random training data
    let num_samples = 100;
    let num_features = 2;
    let features = Array2::random((num_samples, num_features), Uniform::new(-10.0, 10.0));
    let labels = Array1::from_iter(features.outer_iter().map(|row| {
        let sum: f64 = row.iter().sum();
        if sum > 0.0 {
            1.0
        } else {
            0.0
        }
    }));

    // Create and train the classifier
    let mut classifier = Classifier::new(num_features);
    classifier.train(&features, &labels, 1000, 0.01);

    // Generate random test data
    let num_test_samples = 10;
    let test_features = Array2::random((num_test_samples, num_features), Uniform::new(-10.0, 10.0));

    // Predict the labels for the test data
    let test_predictions = classifier.predict(&test_features);

    // Display the test features and their predicted labels
    println!("Test Features:\n{:?}", test_features);
    println!("Predictions:\n{:?}", test_predictions);
}
