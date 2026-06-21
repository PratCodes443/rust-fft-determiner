use crate::fft;
use crate::windows;

/// Short-Time Fourier Transform
pub fn stft(
    signal: &[f64],
    window_size: usize,
    hop_size: usize,
) -> Vec<Vec<f64>> {

    let mut spectrogram =
        Vec::new();

    if signal.len() < window_size {
        return spectrogram;
    }

    let mut pos = 0;

    while pos + window_size <= signal.len() {

        let frame =
            &signal[
                pos..
                pos + window_size
            ];

        let windowed =
            windows::hann(frame);

        let magnitude =
            fft::magnitude(&windowed);

        spectrogram.push(
            magnitude
        );

        pos += hop_size;
    }

    spectrogram
}
