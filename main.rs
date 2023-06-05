fn main() {
    // Example usage of Complex
    let complex_number = Complex::new(3.0, 4.0);
    println!("Complex number: {} + {}i", complex_number.real, complex_number.imaginary);

    // Example usage of Vec3
    let vec3 = Vec3::new(1.0, 2.0, 3.0);
    println!("Vec3: ({}, {}, {})", vec3.x, vec3.y, vec3.z);

    // Example usage of Statistics
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let statistics = Statistics::new(data);
    let mean = statistics.mean();
    println!("Mean: {}", mean);

    // Example usage of Classifier
    let features = vec![vec![1.0, 1.0], vec![2.0, 2.0], vec![3.0, 3.0]];
    let labels = vec![0.0, 0.0, 1.0];
    let mut classifier = Classifier::new(2);
    classifier.train(&features, &labels, 100, 0.1);
    let test_features = vec![vec![4.0, 4.0], vec![5.0, 5.0]];
    let predictions = classifier.predict(&test_features);
    println!("Predictions: {:?}", predictions);
}