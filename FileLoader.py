import streamlit as st
import pandas as pd
import subprocess


st.title("FFT Rust Data Loading")
file_loader = st.empty()
butn = st.empty()


file = file_loader.file_uploader("Upload Data")

if file != None:
    df = pd.read_csv(file)


butn.button("Press To Run")

if butn:
    subprocess.call("cargo run")



