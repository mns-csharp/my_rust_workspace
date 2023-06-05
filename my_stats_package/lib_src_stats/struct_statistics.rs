use std::collections::HashMap;

struct Statistics {
    data: Vec<f64>,
}

impl Statistics {
    fn new(data: Vec<f64>) -> Self {
        Statistics { data }
    }

    fn mean(&self) -> f64 {
        let sum: f64 = self.data.iter().sum();
        sum / self.data.len() as f64
    }

    fn median(&self) -> f64 {
        let mut sorted_data = self.data.clone();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = sorted_data.len() / 2;

        if sorted_data.len() % 2 == 0 {
            (sorted_data[mid - 1] + sorted_data[mid]) / 2.0
        } else {
            sorted_data[mid]
        }
    }

    fn mode(&self) -> Option<f64> {
        let mut counts = HashMap::new();

        for &value in &self.data {
            *counts.entry(value).or_insert(0) += 1;
        }

        let max_count = counts.values().max()?;
        let mode_values: Vec<&f64> = counts
            .iter()
            .filter(|(_, &count)| count == *max_count)
            .map(|(&value, _)| value)
            .collect();

        mode_values.first().cloned()
    }

    fn variance(&self) -> Option<f64> {
        let mean = self.mean();
        let variance = self.data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / self.data.len() as f64;
        Some(variance)
    }

    fn standard_deviation(&self) -> Option<f64> {
        let variance = self.variance()?;
        Some(variance.sqrt())
    }
}


