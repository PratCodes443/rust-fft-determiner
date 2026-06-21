pub fn moving_average(
    signal: &[f64],
    window: usize,
) -> Vec<f64> {

    let mut output = Vec::new();

    for i in 0..signal.len() {

        let start =
            i.saturating_sub(window - 1);

        let slice =
            &signal[start..=i];

        let avg =
            slice.iter().sum::<f64>()
            /
            slice.len() as f64;

        output.push(avg);
    }

    output
}

pub fn ema(
    signal: &[f64],
    alpha: f64,
) -> Vec<f64> {

    let mut result = Vec::new();

    if signal.is_empty() {
        return result;
    }

    result.push(signal[0]);

    for i in 1..signal.len() {

        let value =
            alpha * signal[i]
            +
            (1.0 - alpha)
            * result[i - 1];

        result.push(value);
    }

    result
}

pub fn low_pass(
    signal: &[f64],
    alpha: f64,
) -> Vec<f64> {

    ema(signal, alpha)
}

pub fn high_pass(
    signal: &[f64],
    alpha: f64,
) -> Vec<f64> {

    let mut output =
        vec![0.0; signal.len()];

    for i in 1..signal.len() {

        output[i] =
            alpha
            *
            (
                output[i - 1]
                +
                signal[i]
                -
                signal[i - 1]
            );
    }

    output
}

pub fn median_filter(
    signal: &[f64],
    window: usize,
) -> Vec<f64> {

    let mut output = Vec::new();

    for i in 0..signal.len() {

        let start =
            i.saturating_sub(window / 2);

        let end =
            usize::min(
                i + window / 2 + 1,
                signal.len()
            );

        let mut slice =
            signal[start..end].to_vec();

        slice.sort_by(
            |a,b|
            a.partial_cmp(b).unwrap()
        );

        output.push(
            slice[slice.len()/2]
        );
    }

    output
}

