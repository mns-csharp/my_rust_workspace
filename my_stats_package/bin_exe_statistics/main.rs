fn main() {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 6.0, 7.0, 8.0, 9.0];
    let statistics = Statistics::new(data);

    println!("Mean: {:.2}", statistics.mean());
    println!("Median: {:.2}", statistics.median());
    println!("Mode: {:?}", statistics.mode());
    println!("Variance: {:.2}", statistics.variance().unwrap());
    println!("Standard Deviation: {:.2}", statistics.standard_deviation().unwrap());
}