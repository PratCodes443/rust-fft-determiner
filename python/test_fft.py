import numpy as np
import matplotlib.pyplot as plt
import rust_fft

fs = 1000

t = np.linspace(
    0,
    1,
    fs,
    endpoint=False
)

signal = (
    np.sin(2 * np.pi * 50 * t)
    +
    0.5 * np.sin(2 * np.pi * 120 * t)
)

fft_result = rust_fft.compute_fft_py(
    signal.tolist()
)

magnitude = rust_fft.magnitude(
    signal.tolist()
)

print("\nFFT First 10 Values:\n")
print(fft_result[:10])

plt.figure(figsize=(10, 5))

plt.plot(
    magnitude[:500]
)

plt.title(
    "FFT Magnitude Spectrum"
)

plt.xlabel(
    "Frequency Bin"
)

plt.ylabel(
    "Magnitude"
)

plt.grid(True)

plt.show()