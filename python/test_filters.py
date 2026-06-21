import numpy as np
import matplotlib.pyplot as plt

import rust_fft

fs = 500

t = np.linspace(
    0,
    1,
    fs
)

signal = (
    np.sin(2*np.pi*10*t)
    +
    0.3*np.random.randn(fs)
)

filtered = rust_fft.low_pass_py(
    signal.tolist(),
    0.2
)

plt.figure(figsize=(10,5))

plt.plot(
    signal,
    label="Original"
)

plt.plot(
    filtered,
    label="Low Pass"
)

plt.legend()

plt.show()