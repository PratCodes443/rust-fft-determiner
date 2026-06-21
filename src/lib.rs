pub mod fft;
pub mod statistics;
pub mod windows;
pub mod filters;
pub mod stft;
pub mod utils;


use pyo3::prelude::*;

#[pyfunction]
fn compute_fft_py(
    signal: Vec<f64>
) -> PyResult<Vec<(f64, f64)>> {
    Ok(
        fft::compute_fft(&signal)
    )
}

#[pyfunction]
fn magnitude(
    signal: Vec<f64>
) -> PyResult<Vec<f64>> {
    Ok(
        fft::magnitude(&signal)
    )
}

#[pyfunction]
fn mean(signal: Vec<f64>) -> PyResult<f64> {
    Ok(statistics::mean(&signal))
}

#[pyfunction]
fn rms(signal: Vec<f64>) -> PyResult<f64> {
    Ok(statistics::rms(&signal))
}

#[pyfunction]
fn energy(signal: Vec<f64>) -> PyResult<f64> {
    Ok(statistics::energy(&signal))
}

#[pyfunction]
 fn hann_window(signal: Vec<f64>) -> PyResult<Vec<f64>> {
    Ok(windows::hann(&signal))
}

#[pyfunction]
 fn hamming_window(signal: Vec<f64>) -> PyResult<Vec<f64>> {
    Ok(windows::hamming(&signal))
}

#[pyfunction]
 fn blackman_window(signal: Vec<f64>) -> PyResult<Vec<f64>> {
    Ok(windows::blackman(&signal))
}

#[pyfunction]
fn moving_average_py(
    signal:Vec<f64>,
    window:usize
)->PyResult<Vec<f64>>{

    Ok(
        filters::moving_average(
            &signal,
            window
        )
    )
}

#[pyfunction]
fn ema(
    signal: Vec<f64>,
    alpha: f64,
) -> PyResult<Vec<f64>> {

    Ok(
        filters::ema(
            &signal,
            alpha,
        )
    )
}

#[pyfunction]
pub fn low_pass_py(
    signal: Vec<f64>,
    alpha: f64,
) -> PyResult<Vec<f64>> {

    Ok(
        filters::low_pass(
            &signal,
            alpha,
        )
    )
}

#[pyfunction]
pub fn high_pass_py(
    signal: Vec<f64>,
    alpha: f64,
) -> PyResult<Vec<f64>> {

    Ok(
       filters:: high_pass(
            &signal,
            alpha,
        )
    )
}

#[pyfunction]
pub fn median_filter_py(
    signal: Vec<f64>,
    window: usize,
) -> PyResult<Vec<f64>> {

    Ok(
        filters::median_filter(
            &signal,
            window,
        )
    )
}

#[pyfunction]
pub fn stft_py(
    signal: Vec<f64>,
    window_size: usize,
    hop_size: usize,
) -> PyResult<Vec<Vec<f64>>> {

    Ok(
        stft::stft(
            &signal,
            window_size,
            hop_size,
        )
    )
}

#[pyfunction]
pub fn normalize_py(
    signal: Vec<f64>,
) -> PyResult<Vec<f64>> {

    Ok(utils::normalize(&signal))
}

#[pyfunction]
pub fn zero_pad_py(
    signal: Vec<f64>,
    target_length: usize,
) -> PyResult<Vec<f64>> {

    Ok(
        utils::zero_pad(
            &signal,
            target_length,
        )
    )
}

#[pyfunction]
pub fn remove_dc_py(
    signal: Vec<f64>,
) -> PyResult<Vec<f64>> {

    Ok(utils::remove_dc(&signal))
}


#[pymodule]
fn rust_fft(
    m: &Bound<'_, PyModule>
) -> PyResult<()> {

    m.add_function(
        wrap_pyfunction!(compute_fft_py, m)?
    )?;

    m.add_function(
        wrap_pyfunction!(magnitude, m)?
    )?;

    m.add_function(
        wrap_pyfunction!(mean, m)?
    )?;

    m.add_function(
        wrap_pyfunction!(rms, m)?
    )?;

    m.add_function(
        wrap_pyfunction!(energy, m)?
    )?;

    m.add_function(
        wrap_pyfunction!(hann_window, m)?
    )?;

    m.add_function(
        wrap_pyfunction!(hamming_window, m)?
    )?;

    m.add_function(
        wrap_pyfunction!(blackman_window, m)?
    )?;

    m.add_function(
        wrap_pyfunction!(moving_average_py, m)?
    )?;

    m.add_function(
        wrap_pyfunction!(ema, m)?
    )?;

    m.add_function(
        wrap_pyfunction!(low_pass_py, m)?
    )?;

    m.add_function(
        wrap_pyfunction!(high_pass_py, m)?
    )?;

    m.add_function(
        wrap_pyfunction!(median_filter_py, m)?
    )?;

    m.add_function(
        wrap_pyfunction!(stft_py, m)?
    )?;

    m.add_function(
        wrap_pyfunction!(normalize_py, m)?
    )?;
    
    m.add_function(
        wrap_pyfunction!(zero_pad_py, m)?
    )?;

    m.add_function(
        wrap_pyfunction!(remove_dc_py, m)?
    )?;



    Ok(())
}