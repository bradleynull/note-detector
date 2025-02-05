mod notes;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rustfft::{num_complex::Complex, FftPlanner};
use strum::IntoEnumIterator;
use notes::Note;

fn main() {
    // Get the default host
    let host = cpal::default_host();

    // Get the default input device
    let device = host
        .default_input_device()
        .expect("Failed to get default input device");

    // Get the default input format
    let supported_config = device
        .default_input_config()
        .expect("Failed to get default input format");
    let format = supported_config.config();

    // Determine the basics for the FFT
    let sample_rate = format.sample_rate.0 as f32;
    let fft_size = 1024;
    let frequency_resolution = sample_rate / fft_size as f32;

    // Setup the FFT
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(fft_size);
    let sample_buffer = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

    // Create a stream to capture audio
    let stream = {
        let sample_buffer = std::sync::Arc::clone(&sample_buffer);
        device
            .build_input_stream(
                &format,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    let mut buffer = sample_buffer.lock().unwrap();
                    buffer.extend_from_slice(data);

                    if buffer.len() >= fft_size {
                        let mut input: Vec<Complex<f32>> = buffer
                            .drain(..fft_size)
                            .map(|x| Complex { re: x, im: 0.0 })
                            .collect();
                        fft.process(&mut input);

                        // Determine the magnitudes of the FFT output
                        let magnitudes: Vec<f32> = input.iter().map(|c| c.norm()).collect();

                        // Determine the most pronounced note
                        let mut max_magnitude = 0.0;
                        let mut most_pronounced_note = None;

                        for note in Note::iter() {
                            let target_index =
                                (note.frequency() / frequency_resolution).round() as usize;
                            if target_index < magnitudes.len()
                                && magnitudes[target_index] > max_magnitude
                            {
                                max_magnitude = magnitudes[target_index];
                                most_pronounced_note = Some(note);
                            }
                        }

                        if let Some(note) = most_pronounced_note {
                            print!("\x1B[2J\x1B[H");
                            println!(
                                "Most pronounced note: {:?} with magnitude: {}",
                                note, max_magnitude
                            );
                        }
                    }
                },
                move |err| {
                    // Handle errors here
                    eprintln!("Error: {:?}", err);
                },
                None, // Add the fourth argument for input stream configuration options
            )
            .expect("Failed to build input stream")
    };

    // Start the stream
    stream.play().expect("Failed to play stream");

    std::thread::sleep(std::time::Duration::from_secs(10));

    println!("Done...");
}
