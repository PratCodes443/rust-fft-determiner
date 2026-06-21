import numpy as np
import matplotlib.pyplot as plt
import rust_fft

# ----------------------------
# User Input
# ----------------------------

freqs = list(
    map(
        float,
        input(
            "Enter frequencies (space separated): "
        ).split()
    )
)

sampling_rate = int(
    input(
        "Sampling Rate [1000]: "
    ) or "1000"
)

duration = float(
    input(
        "Duration [1.0 sec]: "
    ) or "1.0"
)

# ----------------------------
# Generate Signal
# ----------------------------

t = np.linspace(
    0,
    duration,
    int(
        sampling_rate * duration
    ),
    endpoint=False
)

signal = np.zeros_like(t)

for freq in freqs:

    signal += np.sin(
        2 * np.pi *
        freq * t
    )

signal_list = signal.tolist()

# ----------------------------
# Statistics
# ----------------------------

print("\n=== Statistics ===")

print(
    "Mean:",
    rust_fft.mean(signal_list)
)

print(
    "RMS:",
    rust_fft.rms(signal_list)
)

print(
    "Energy:",
    rust_fft.energy(signal_list)
)

# ----------------------------
# FFT
# ----------------------------

magnitude = rust_fft.magnitude(
    signal_list
)

# ----------------------------
# Filters
# ----------------------------

moving_avg = rust_fft.moving_average_py(
    signal_list,
    5
)

low_pass = rust_fft.low_pass_py(
    signal_list,
    0.2
)

# ----------------------------
# STFT
# ----------------------------

stft_result = rust_fft.stft_py(
    signal_list,
    128,
    64
)

spectrogram = np.array(
    stft_result
)

# ----------------------------
# Plot Results
# ----------------------------

plt.figure(
    figsize=(12, 8)
)

# Original Signal

plt.subplot(2, 2, 1)

plt.plot(signal)

plt.title(
    "Generated Signal"
)

plt.grid(True)

# FFT

plt.subplot(2, 2, 2)

plt.plot(
    magnitude[:len(magnitude)//2]
)

plt.title(
    "FFT Magnitude Spectrum"
)

plt.grid(True)

# Filters

plt.subplot(2, 2, 3)

plt.plot(
    signal,
    label="Original",
    alpha=0.5
)

plt.plot(
    moving_avg,
    label="Moving Average"
)

plt.plot(
    low_pass,
    label="Low Pass"
)

plt.legend()

plt.title(
    "Filtering"
)

plt.grid(True)

# STFT

plt.subplot(2, 2, 4)

plt.imshow(
    spectrogram.T,
    aspect="auto",
    origin="lower"
)

plt.title(
    "STFT Spectrogram"
)

plt.tight_layout()

plt.show()