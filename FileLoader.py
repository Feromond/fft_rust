import streamlit as st
import pandas as pd
import subprocess
import os

def check_file(file):
    # Check if the file extension is valid
    if file.type not in ["text/csv", "text/plain"]:
        return False, "File type not supported. Please upload a .txt or .csv file."
    
    # Load the file into a DataFrame
    df = pd.read_csv(file, header=None)
    
    # If everything checks out
    return True, df

st.set_page_config(page_title="FFT Rust - File Loader", layout="wide")
st.title("FFT Rust Data Loading")
file_loader = st.empty()

file = file_loader.file_uploader("Upload Data")

uploaded_file_path = None
if file is not None:
    is_valid, result = check_file(file)
    if is_valid:
        # Write the DataFrame to a file in the temporary_uploads directory
        os.makedirs('temporary_uploads', exist_ok=True)
        uploaded_file_path = f'temporary_uploads/{file.name}'
        result.to_csv(uploaded_file_path, index=False, header=False)
        butn = st.button("Press To Run", disabled=False, key=2)
    else:
        st.error(result)
        st.stop()
else:
    st.stop()

st.divider()

if 'butn' in locals() and butn:
    st.write(f"./{uploaded_file_path}")
    subprocess.call(["./target/release/fft_rust.exe", f"./{uploaded_file_path}"])
    col1, col2 = st.columns([1,1])
    col1.image("output_figures/raw_signal.png")
    col2.image("output_figures/amplitude_spectrum.png")
    os.remove(f"./{uploaded_file_path}")