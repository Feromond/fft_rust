import streamlit as st
import pandas as pd
import subprocess



def check_file(file):
    # Check if the file extension is valid
    if file.type not in ["text/csv", "text/plain"]:
        return False, "File type not supported. Please upload a .txt or .csv file."
    
    # Check if the file has exactly two columns
    df = pd.read_csv(file, header=None)
    # if df.shape[1] != 2:
    #     return False, "Invalid file format. Please upload a file with exactly two columns."
    
    # If everything checks out
    return True, df


st.set_page_config(page_title="FFT Rust - File Loader", layout="wide")
st.title("FFT Rust Data Loading")
file_loader = st.empty()

file = file_loader.file_uploader("Upload Data")

if file is not None:
    is_valid, result = check_file(file)
    if is_valid:
        df = result
        butn = st.button("Press To Run", disabled=False, key=2)
    else:
        st.error(result)
        st.stop()
else:
    st.stop()

st.divider()

if 'butn' in locals() and butn:
    subprocess.call(["./target/release/fft_rust", f"./{file.name}"])
    col1, col2 = st.columns([1,1])
    col1.image("raw_signal.png")
    col2.image("amplitude_spectrum.png")