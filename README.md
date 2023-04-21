# fft_rust
## Hobby Project By: *Jacob Mish*

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
    </li>
    <li>
      <a href="#release-notes">Release Notes</a>
      <ul>
        <li><a href="#pre-release-v1">Pre-v1</a></li>
      </ul>
    </li>
    <li>
      <a href="#references">References</a>
    </li>
    
    
  </ol>
</details>

## About The Project

This project aims to create a dynamic fast fourier transform tool which can plot any raw signal and the associated amplitude spectrum after FFT.


## Release Notes:

#### Pre-release v1:

![alt text](https://github.com/Feromond/fft_rust/blob/master/raw_signal.png?raw=true)

- Created a base simple version of the program with static initial inputs to test and validate the program functions. 
- Plotting functionality is able to dynamically adjust plot x and y dimensions based on the input data, sampling rates, and number of points.

- Future implementation will begin to add forms of user input or potential implement basic file reading to process more interesting data.

#### Pre-release v2:

- Updated project to dynamically update plot windows and axis based on input data.
- Changed the signal from a simple pre-configured sinusoid to instead be read in from a file containing some complex raw signal.
- Adjusted all static parameters to dynamically compute based on the input raw signal file.

- Future implementations will start to work with the users inputs and will also handle more filetypes better.
- Theoretical future implementation will be to work on handling errors and exceptions better, as well as overall code optimization and redundancy reduction.


### References

[Rustfft](https://docs.rs/rustfft/latest/rustfft/)

