pub fn mean(signal: &[f64]) -> f64 {
    signal.iter().sum::<f64>()
        / signal.len() as f64
}

pub fn rms(signal: &[f64]) -> f64 {
    (
        signal.iter()
            .map(|x| x * x)
            .sum::<f64>()
            / signal.len() as f64
    ).sqrt()
}

pub fn energy(signal: &[f64]) -> f64 {
    signal.iter()
        .map(|x| x * x)
        .sum::<f64>()
}