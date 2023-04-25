import streamlit as st
import pandas as pd
import subprocess


st.title("FFT Rust Data Loading")
file_loader = st.empty()


file = file_loader.file_uploader("Upload Data")


if file != None:
    df = pd.read_csv(file, header=None)
    st.dataframe(df)
    butn = st.button("Press To Run", disabled=False, key=2)
elif file == None:
    st.stop()


if butn:
    subprocess.call("./fft_rust")

    st.image("raw_signal.png")
    st.divider()
    st.image("amplitude_spectrum.png")



