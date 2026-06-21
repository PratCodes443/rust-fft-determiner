import streamlit as st
import plotly.graph_objects as go
import numpy as np
import rust_fft

st.set_page_config(
    page_title="Rust FFT Determiner",
    layout="wide"
)

st.title("Rust FFT Determiner")

st.sidebar.header("Signal Parameters")

frequency = st.sidebar.slider(
    "Frequency (Hz)",
    min_value=1,
    max_value=1000,
    value=50
)

amplitude = st.sidebar.slider(
    "Amplitude",
    min_value=1.0,
    max_value=10.0,
    value=1.0
)

sampling_rate = st.sidebar.selectbox(
    "Sampling Rate",
    [500, 1000, 2000, 5000],
    index=1
)

duration = st.sidebar.slider(
    "Duration (seconds)",
    min_value=1,
    max_value=10,
    value=1
)

# Generate Signal

t = np.linspace(
    0,
    duration,
    int(sampling_rate * duration),
    endpoint=False
)

signal = amplitude * np.sin(
    2 * np.pi * frequency * t
)

signal_list = signal.tolist()

# --------------------------
# Statistics
# --------------------------

mean = rust_fft.mean(signal_list)
rms = rust_fft.rms(signal_list)
energy = rust_fft.energy(signal_list)

col1, col2, col3 = st.columns(3)

col1.metric("Mean", f"{mean:.4f}")
col2.metric("RMS", f"{rms:.4f}")
col3.metric("Energy", f"{energy:.4f}")

# --------------------------
# Waveform
# --------------------------

st.subheader("Signal Waveform")

wave_fig = go.Figure()

wave_fig.add_trace(
    go.Scatter(
        x=t,
        y=signal,
        mode="lines",
        name="Signal"
    )
)

wave_fig.update_layout(
    height=400
)

st.plotly_chart(
    wave_fig,
    use_container_width=True
)

# --------------------------
# FFT
# --------------------------

st.subheader("FFT Magnitude Spectrum")

magnitude = rust_fft.magnitude(
    signal_list
)

freq_axis = np.fft.fftfreq(
    len(signal),
    d=1 / sampling_rate
)

fft_fig = go.Figure()

fft_fig.add_trace(
    go.Scatter(
        x=freq_axis[:len(freq_axis)//2],
        y=magnitude[:len(magnitude)//2],
        mode="lines",
        name="FFT"
    )
)

fft_fig.update_layout(
    height=400
)

st.plotly_chart(
    fft_fig,
    use_container_width=True
)

# --------------------------
# STFT
# --------------------------

st.subheader("STFT Spectrogram")

stft_result = rust_fft.stft_py(
    signal_list,
    128,
    64
)

if len(stft_result) > 0:

    spec = np.array(
        stft_result
    )

    heatmap = go.Figure(
        data=go.Heatmap(
            z=spec.T
        )
    )

    heatmap.update_layout(
        height=500
    )

    st.plotly_chart(
        heatmap,
        use_container_width=True
    )

# --------------------------
# Raw Data
# --------------------------

with st.expander(
    "Show Raw Signal Samples"
):

    st.write(
        signal[:100]
    )