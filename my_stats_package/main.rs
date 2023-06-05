fn main() {
    // Vec3 demonstration
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);

    // Calculate dot product of two vectors
    let dot_product = v1.dot(v2);
    println!("Dot Product: {}", dot_product);

    // Calculate cross product of two vectors
    let cross_product = v1.cross(v2);
    println!("Cross Product: {:?}", cross_product);

    // Complex demonstration
    let c1 = Complex::new(1.0, 2.0);
    let c2 = Complex::new(3.0, 4.0);

    // Add two complex numbers
    let sum = c1.add(c2);
    println!("Sum: {:?}", sum);

    // Subtract two complex numbers
    let diff = c1.subtract(c2);
    println!("Difference: {:?}", diff);

    // Multiply two complex numbers
    let product = c1.multiply(c2);
    println!("Product: {:?}", product);

    // Divide two complex numbers
    let quotient = c1.divide(c2);
    println!("Quotient: {:?}", quotient);
}