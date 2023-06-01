# FFT_Rust
## Hobby Project By: *Jacob Mish*

<!-- TABLE OF CONTENTS -->

  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#release-notes">Release Notes</a>
      <ul>
        <li><a href="#pre-release-v1">Pre-v1</a></li>
        <li><a href="#pre-release-v2">Pre-v2</a></li>
        <li><a href="#alpha-release-v1">Alpha_v1</a></li>
      </ul>
    </li>
    <li>
      <a href="#references">References</a>
    </li>
    
    
  </ol>


## About The Project

This project aims to create a dynamic fast fourier transform tool which can plot any raw signal and the associated amplitude spectrum after FFT.


## Release Notes:

#### Pre-release v1:


<p align="center">
  <img width="320" height="240" src="https://github.com/Feromond/fft_rust/blob/master/output_figures/raw_signal.png?raw=true">
  <img width="320" height="240" src="https://github.com/Feromond/fft_rust/blob/master/output_figures/amplitude_spectrum.png?raw=true">
  
</p>


- Created a base simple version of the program with static initial inputs to test and validate the program functions. 
- Plotting functionality is able to dynamically adjust plot x and y dimensions based on the input data, sampling rates, and number of points.

- Future implementation will begin to add forms of user input or potential implement basic file reading to process more interesting data.

#### Pre-release v2:

- Updated project to dynamically update plot windows and axis based on input data.
- Changed the signal from a simple pre-configured sinusoid to instead be read in from a file containing some complex raw signal.
- Adjusted all static parameters to dynamically compute based on the input raw signal file.

- Future implementations will start to work with the users inputs and will also handle more filetypes better.
- Theoretical future implementation will be to work on handling errors and exceptions better, as well as overall code optimization and redundancy reduction.

#### Alpha-release v1:

- Utilized python streamlit for the front-end of the application to allow for file uploading and displaying of the results
- Currently do not have much error handling for invalid data structures within files.
- Columns must still be in specific order for the data to plot correctly, hopefully in a future version I will add a way to self identify which column is most likely to be the "time" and which is the measured amplitudes.
- Added argument handling so when running the binary, a specific file can be referenced and it will process it. Still defaults to "sample_dataset/sample_data.txt" if there is no argument provided. Will not run without that file in the correct directory.

**Run the Streamlit Front-End Version**: Clone this repo locally and then ensure you have a python version with streamlit installed. Then within the project directory write 
```
streamlit run FileLoader.py
```

If there is some error occuring, a temporary solution could be to recompile the rust binary release locally. This can be done within the project directory using
```
cargo build --release
```

### References

[Rustfft](https://docs.rs/rustfft/latest/rustfft/)

