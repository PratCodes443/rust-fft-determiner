use rust_fft::statistics;
use rust_fft::filters;
use rust_fft::utils;

#[test]
fn test_mean() {
    let signal = vec![1.0, 2.0, 3.0, 4.0];

    assert_eq!(
        statistics::mean(&signal),
        2.5
    );
}

#[test]
fn test_energy() {
    let signal = vec![1.0, 2.0, 3.0];

    assert_eq!(
        statistics::energy(&signal),
        14.0
    );
}

#[test]
fn test_normalize() {

    let signal =
        vec![10.0, 20.0, 30.0];

    let result =
        utils::normalize(&signal);

    assert_eq!(result[0], 0.0);
    assert_eq!(result[2], 1.0);
}

#[test]
fn test_moving_average() {

    let signal =
        vec![1.0,2.0,3.0,4.0];

    let result =
        filters::moving_average(
            &signal,
            2
        );

    assert_eq!(
        result.len(),
        4
    );
}