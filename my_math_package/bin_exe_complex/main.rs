fn main() {
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