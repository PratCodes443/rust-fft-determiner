use std::f64::consts::PI;
/// Hann Window
pub fn hann(signal: &[f64]) -> Vec<f64> {
    let n = signal.len();

    signal
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            let w = 0.5
                - 0.5
                    * ((2.0 * PI * i as f64)
                        / (n - 1) as f64)
                        .cos();

            x * w
        })
        .collect()
}

/// Hamming Window
pub fn hamming(signal: &[f64]) -> Vec<f64> {
    let n = signal.len();

    signal
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            let w = 0.54
                - 0.46
                    * ((2.0 * PI * i as f64)
                        / (n - 1) as f64)
                        .cos();

            x * w
        })
        .collect()
}

/// Blackman Window
pub fn blackman(signal: &[f64]) -> Vec<f64> {
    let n = signal.len();

    signal
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            let w = 0.42
                - 0.5
                    * ((2.0 * PI * i as f64)
                        / (n - 1) as f64)
                        .cos()
                + 0.08
                    * ((4.0 * PI * i as f64)
                        / (n - 1) as f64)
                        .cos();

            x * w
        })
        .collect()
}