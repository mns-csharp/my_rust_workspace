macro_rules! print_complex {
    ($complex:expr) => {
        println!("Complex number: {} + {}i", $complex.real, $complex.imaginary);
    };
}