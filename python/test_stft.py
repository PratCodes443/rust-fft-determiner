import numpy as np
import matplotlib.pyplot as plt

import rust_fft

fs = 1000

t = np.linspace(
    0,
    2,
    fs*2,
    endpoint=False
)

signal = np.concatenate([
    np.sin(2*np.pi*50*t[:1000]),
    np.sin(2*np.pi*150*t[1000:])
])

spec = rust_fft.stft_py(
    signal.tolist(),
    128,
    64
)

spec = np.array(spec)

plt.figure(figsize=(10,6))

plt.imshow(
    spec.T,
    aspect="auto",
    origin="lower"
)

plt.colorbar()

plt.title(
    "STFT Spectrogram"
)

plt.xlabel("Frame")
plt.ylabel("Frequency Bin")

plt.show()