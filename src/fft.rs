use rustfft::FftPlanner;
use rustfft::num_complex::Complex;

pub fn compute_fft(
    signal: &[f64]
) -> Vec<(f64,f64)> {

    let mut planner = FftPlanner::<f64>::new();

    let fft =
        planner.plan_fft_forward(signal.len());

    let mut buffer: Vec<Complex<f64>> =
        signal.iter()
        .map(|&x| Complex {
            re:x,
            im:0.0
        })
        .collect();

    fft.process(&mut buffer);

    buffer
        .iter()
        .map(|c| (c.re,c.im))
        .collect()
}

pub fn magnitude(
    signal:&[f64]
)->Vec<f64>{

    let fft_result =
        compute_fft(signal);

    fft_result
        .iter()
        .map(|(re,im)|{

            (re*re + im*im).sqrt()

        })
        .collect()
}