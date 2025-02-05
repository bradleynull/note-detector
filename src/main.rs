use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rustfft::{num_complex::Complex, FftPlanner};
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Copy, Clone, EnumIter, Debug)]
enum Note {
    G3 = 196,
    G3Sharp = 207,
    A3 = 220,
    A3Sharp = 233,
    B3 = 247,
    C4 = 262,
    C4Sharp = 277,
    D4 = 294,
    D4Sharp = 311,
    E4 = 330,
    F4 = 349,
    F4Sharp = 370,
    G4 = 392,
    G4Sharp = 415,
    A4 = 440,
    A4Sharp = 466,
    B4 = 494,
    C5 = 523,
    C5Sharp = 554,
    D5 = 587,
    D5Sharp = 622,
    E5 = 659,
    F5 = 698,
    F5Sharp = 740,
    G5 = 784,
    G5Sharp = 831,
    A5 = 880,
    A5Sharp = 932,
    B5 = 988,
    C6 = 1047,
    C6Sharp = 1109,
    D6 = 1175,
    D6Sharp = 1245,
    E6 = 1319,
    F6 = 1397,
    F6Sharp = 1480,
    G6 = 1568,
    G6Sharp = 1661,
    A6 = 1760,
    A6Sharp = 1865,
    B6 = 1976,
    C7 = 2093,
    C7Sharp = 2217,
    D7 = 2349,
    D7Sharp = 2489,
    E7 = 2637,
}

impl Note {
    fn frequency(&self) -> f32 {
        *self as i32 as f32
    }
}

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

    // Set up Ctrl+C handler
    let running = std::sync::Arc::new(AtomicBool::new(true));
    {
        let running = running.clone();
        ctrlc::set_handler(move || {
            println!("Ctrl+C pressed, terminating...");
            running.store(false, Ordering::SeqCst);
        }).expect("Error setting Ctrl-C handler");
    }

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

    // Keep the application running to capture audio
    while running.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    println!("Terminating...");
}
