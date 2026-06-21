import rust_fft
import numpy as np

def get_signal():

    print("\nEnter signal values separated by spaces:")

    signal = list(
        map(
            float,
            input("> ").split()
        )
    )

    return signal


while True:

    print("\n===== Rust DSP Toolkit =====")

    print("1. FFT")
    print("2. Magnitude Spectrum")
    print("3. Mean")
    print("4. RMS")
    print("5. Energy")
    print("6. Hann Window")
    print("7. Hamming Window")
    print("8. Blackman Window")
    print("9. Moving Average")
    print("10. EMA")
    print("11. Low Pass")
    print("12. High Pass")
    print("13. Median Filter")
    print("14. Normalize")
    print("15. Remove DC")
    print("16. Zero Pad")
    print("17. STFT")
    print("0. Exit")

    choice = int(input("\nChoice: "))

    if choice == 0:
        break

    signal = get_signal()

    if choice == 1:

        result = rust_fft.compute_fft_py(signal)

        print(result)

    elif choice == 2:

        result = rust_fft.magnitude(signal)

        print(result)

    elif choice == 3:

        print(
            rust_fft.mean(signal)
        )

    elif choice == 4:

        print(
            rust_fft.rms(signal)
        )

    elif choice == 5:

        print(
            rust_fft.energy(signal)
        )

    elif choice == 6:

        print(
            rust_fft.hann_window(signal)
        )

    elif choice == 7:

        print(
            rust_fft.hamming_window(signal)
        )

    elif choice == 8:

        print(
            rust_fft.blackman_window(signal)
        )

    elif choice == 9:

        window = int(
            input(
                "Window Size: "
            )
        )

        print(
            rust_fft.moving_average_py(
                signal,
                window
            )
        )

    elif choice == 10:

        alpha = float(
            input(
                "Alpha (0-1): "
            )
        )

        print(
            rust_fft.ema_py(
                signal,
                alpha
            )
        )

    elif choice == 11:

        alpha = float(
            input(
                "Alpha (0-1): "
            )
        )

        print(
            rust_fft.low_pass_py(
                signal,
                alpha
            )
        )

    elif choice == 12:

        alpha = float(
            input(
                "Alpha (0-1): "
            )
        )

        print(
            rust_fft.high_pass_py(
                signal,
                alpha
            )
        )

    elif choice == 13:

        window = int(
            input(
                "Window Size: "
            )
        )

        print(
            rust_fft.median_filter_py(
                signal,
                window
            )
        )

    elif choice == 14:

        print(
            rust_fft.normalize_py(
                signal
            )
        )

    elif choice == 15:

        print(
            rust_fft.remove_dc_py(
                signal
            )
        )

    elif choice == 16:

        target = int(
            input(
                "Target Length: "
            )
        )

        print(
            rust_fft.zero_pad_py(
                signal,
                target
            )
        )

    elif choice == 17:

        window_size = int(
            input(
                "Window Size: "
            )
        )

        hop_size = int(
            input(
                "Hop Size: "
            )
        )

        result = rust_fft.stft_py(
            signal,
            window_size,
            hop_size
        )

        print(result)

    else:

        print(
            "Invalid Choice"
        )