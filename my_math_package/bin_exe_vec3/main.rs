fn main() {
    // Create two vectors
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);

    // Add the vectors
    let sum = v1 + v2;
    println!("Sum: {:?}", sum);

    // Subtract the vectors
    let diff = v2 - v1;
    println!("Difference: {:?}", diff);

    // Multiply the vector by a scalar
    let mut scaled = Vec3::new(2.0, 3.0, 4.0);
    scaled.mul(2.0);
    println!("Scaled: {:?}", scaled);

    // Divide the vector by a scalar
    let mut divided = Vec3::new(4.0, 6.0, 8.0);
    divided.div(2.0);
    println!("Divided: {:?}", divided);

    // Calculate the length squared
    let length_squared = v1.length_squared();
    println!("Length squared: {}", length_squared);

    // Normalize a vector
    let normalized = v1.normalized();
    println!("Normalized: {:?}", normalized);
}
