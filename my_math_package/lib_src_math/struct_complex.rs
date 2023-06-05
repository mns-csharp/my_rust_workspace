
#[derive(Debug, Copy, Clone)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    fn new(real: f64, imaginary: f64) -> Complex {
        Complex { real, imaginary }
    }
}

impl TComplex for Complex {
    fn add(&self, other: Complex) -> Complex {
        let real = self.real + other.real;
        let imaginary = self.imaginary + other.imaginary;
        Complex::new(real, imaginary)
    }

    fn subtract(&self, other: Complex) -> Complex {
        let real = self.real - other.real;
        let imaginary = self.imaginary - other.imaginary;
        Complex::new(real, imaginary)
    }

    fn multiply(&self, other: Complex) -> Complex {
        let real = self.real * other.real - self.imaginary * other.imaginary;
        let imaginary = self.real * other.imaginary + self.imaginary * other.real;
        Complex::new(real, imaginary)
    }

    fn divide(&self, other: Complex) -> Complex {
        let denominator = other.real * other.real + other.imaginary * other.imaginary;
        let real = (self.real * other.real + self.imaginary * other.imaginary) / denominator;
        let imaginary = (self.imaginary * other.real - self.real * other.imaginary) / denominator;
        Complex::new(real, imaginary)
    }
}
