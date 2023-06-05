extern crate ndarray;
extern crate ndarray_rand;

use ndarray::{Array1, Array2, ArrayBase, Data, Ix1, Ix2, OwnedRepr};
use ndarray_rand::RandomExt;
use rand::distributions::Uniform;
use std::convert::TryInto;

struct Classifier {
    weights: Array1<f64>,
}

impl Classifier {
    fn new(num_features: usize) -> Self {
        let weights = Array1::random(num_features, Uniform::new(-1.0, 1.0));
        Classifier { weights }
    }

    fn train<T>(&mut self, features: &ArrayBase<T, Ix2>, labels: &ArrayBase<T, Ix1>, num_iterations: usize, learning_rate: f64)
        where
            T: Data<Elem = f64>,
    {
        for _ in 0..num_iterations {
            let predictions = self.predict(features);
            let errors = labels - &predictions;
            let gradient = features.t().dot(&errors) / (features.rows() as f64);
            self.weights += learning_rate * gradient;
        }
    }

    fn predict<T>(&self, features: &ArrayBase<T, Ix2>) -> Array1<f64>
        where
            T: Data<Elem = f64>,
    {
        features.dot(&self.weights)
            .mapv(|x| if x > 0.0 { 1.0 } else { 0.0 })
    }
}

