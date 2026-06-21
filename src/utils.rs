/// Min-Max Normalization
pub fn normalize(signal: &[f64]) -> Vec<f64> {
    if signal.is_empty() {
        return Vec::new();
    }

    let min = signal
        .iter()
        .fold(f64::INFINITY, |a, &b| a.min(b));

    let max = signal
        .iter()
        .fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    if (max - min).abs() < f64::EPSILON {
        return vec![0.0; signal.len()];
    }

    signal
        .iter()
        .map(|&x| (x - min) / (max - min))
        .collect()
}

/// Zero Padding
pub fn zero_pad(
    signal: &[f64],
    target_length: usize,
) -> Vec<f64> {

    let mut padded = signal.to_vec();

    if target_length > padded.len() {
        padded.resize(target_length, 0.0);
    }

    padded
}

/// Remove DC Component
pub fn remove_dc(signal: &[f64]) -> Vec<f64> {

    if signal.is_empty() {
        return Vec::new();
    }

    let mean =
        signal.iter().sum::<f64>()
        /
        signal.len() as f64;

    signal
        .iter()
        .map(|&x| x - mean)
        .collect()
}
