use plotters::prelude::*;
use rustfft::FftPlanner;
use num_complex::Complex;
// use std::f64::consts::PI;
use rustfft::num_traits::Zero;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
// use csv::ReaderBuilder;
fn main() -> Result<(), Box<dyn std::error::Error>> {


    let mut args = env::args();
    let default_path = "sample_data.txt";
    let path = args.nth(1).unwrap_or_else(|| default_path.to_string());
    let mut data = Vec::new();
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                let columns: Vec<&str> = line.split(',').collect();

                if columns.len() >= 2 {
                    data.push(line.to_string());
                }
            }
        }
    }

    let mut data_x:Vec<f64> = Vec::new();
    let mut data_y:Vec<f64> = Vec::new();
    for line in &data{
        let columns: Vec<&str> = line.trim().split(',').collect();
        data_x.push(columns[1].replace(" ", "").parse().unwrap());
        data_y.push(columns[0].replace(" ", "").parse().unwrap());
    }


    // Defining current static controls
    let curve_points: usize = data_x.len();
    let sampling_rate: f64 = data_x[1] - data_x[0];


    let x_values:Vec<f64> = data_x;
    let y_values:Vec<f64> = data_y;

    let mut min_value_y: f64 = y_values
        .iter()
        .cloned()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let mut max_value_y: f64 = y_values
        .iter()
        .cloned()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();


    // Changing the plot y limits to include 10% more values from the minimum / maximum values in the array
    let min_y_10_percent: f64 = 0.1 * min_value_y;
    min_value_y += min_y_10_percent;

    let max_y_10_percent: f64 = 0.1 * max_value_y;
    max_value_y += max_y_10_percent;


    let root_area = BitMapBackend::new("raw_signal.png", (1280, 720)).into_drawing_area();
    root_area.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root_area)
        .caption("Time Domain Signal", ("Arial", 24).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..(sampling_rate * curve_points as f64), min_value_y..max_value_y)?;

    chart.configure_mesh().draw()?;
    
    chart.draw_series(LineSeries::new(
        x_values.iter().zip(y_values.iter()).map(|(&x, &y)| (x, y)),
        &RED,
    ))?;

    let mut planner: FftPlanner<f64> = FftPlanner::new();
    let fft = planner.plan_fft_forward(curve_points);

    let mut signal: Vec<Complex<f64>> = y_values.iter().map(|&y| Complex::new(y, 0.0)).collect();
    let mut scratch: Vec<Complex<f64>> = vec![Complex::default(); fft.get_inplace_scratch_len()];
    fft.process_with_scratch(&mut signal, &mut scratch);
    let mut spectrum: Vec<Complex<f64>> = signal;
    
    let spectrum_shifted: Vec<Complex<f64>> = fft_shift(&mut spectrum);

    let frequencies: Vec<f64> = (-(curve_points as i64) / 2..(curve_points as i64) / 2)
        .map(|i| (i as f64) / (sampling_rate * curve_points as f64))
        .collect();
    
    let magnitudes: Vec<f64> = spectrum_shifted.iter()
    .map(|c| (c.norm() * 2.0) / curve_points as f64)
    .collect();

    // Setting top range for plot window scaling to be max value + 10%
    let mut max_mag_y: f64 = magnitudes
        .iter()
        .cloned()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_mag_y_10_percent: f64 = 0.1 * max_mag_y;
    max_mag_y += max_mag_y_10_percent;

    let root_area = BitMapBackend::new("amplitude_spectrum.png", (1280, 720)).into_drawing_area();
    root_area.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root_area)
        .caption("Amplitude Spectrum of Signal (FFT)", ("Arial", 24).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d((-(curve_points as f64) / 2.0) / (sampling_rate * curve_points as f64)..((curve_points as f64) / 2.0) / (sampling_rate * curve_points as f64), 0f64..max_mag_y)?;

    chart.configure_mesh().draw()?; 

    chart.draw_series(LineSeries::new(
        frequencies.iter().zip(magnitudes.iter()).map(|(&x, &y)| (x, y)),
        &BLUE,
    ))?;
    
    root_area.present()?;
    println!("raw_signal.png and amplitude_spectrum.png created.");
    
    Ok(())
    }
    


fn fft_shift<T: Clone + Zero>(data: &[T]) -> Vec<T> {
    let n = data.len();
    let mut shifted_data: Vec<T> = vec![T::zero(); n];

    let (left, right) = data.split_at(n / 2);
    shifted_data[..n / 2].clone_from_slice(right);
    shifted_data[n / 2..].clone_from_slice(left);

    return shifted_data;
}