use plotters::prelude::*;
use rustfft::FftPlanner;
use num_complex::Complex;
use std::f64::consts::PI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let curve_points: usize = 5000;
    let sampling_rate: f64 = 0.05;
    let frequency: f64 = 1.0 / sampling_rate;
    let frequency1: f64 = 50.0;
    let frequency2: f64 = 80.0;


    let x_values: Vec<f64> = (0..curve_points)
        .map(|i| i as f64 * (1.0 / sampling_rate))
        .collect();
    let y_values: Vec<f64> = x_values
        .iter()
        .map(|&x| ((2.0 * PI * 1.0/frequency1 * x).sin() + (2.0 * PI * 1.0/frequency2 * x).sin()))
        .collect();

    let root_area = BitMapBackend::new("simple_curve.png", (640, 480)).into_drawing_area();
    root_area.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root_area)
        .caption("Simple Curve", ("Arial", 24).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..(1.0 / frequency * 2.0), -4.2f64..4.2f64)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        x_values.iter().zip(y_values.iter()).map(|(&x, &y)| (x, y)),
        &RED,
    ))?;

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(curve_points);

    let mut signal: Vec<Complex<f64>> = y_values.iter().map(|&y| Complex::new(y, 0.0)).collect();
    let mut scratch = vec![Complex::default(); fft.get_inplace_scratch_len()];
    fft.process_with_scratch(&mut signal, &mut scratch);
    let spectrum = signal;
    
    let frequencies: Vec<f64> = (0..curve_points / 2)
        .map(|i| (i as f64) * (sampling_rate / curve_points as f64))
        .collect();
    let magnitudes: Vec<f64> = spectrum[0..(curve_points / 2)]
        .iter()
        .map(|c| (c.norm() * 2.0) / curve_points as f64)
        .collect();

    let root_area = BitMapBackend::new("fft_plot.png", (640, 480)).into_drawing_area();
    root_area.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root_area)
        .caption("FFT of Simple Curve", ("Arial", 24).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..(sampling_rate / 2.0), 0f64..1.2f64)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        frequencies.iter().zip(magnitudes.iter()).map(|(&x, &y)| (x, y)),
        &BLUE,
    ))?;
    
    root_area.present()?;
    println!("simple_curve.png and fft_plot.png created.");
    
    Ok(())
    }
    
