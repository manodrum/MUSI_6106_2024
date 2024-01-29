use std::{fs::File, io::Write};
use hound::WavReader;

fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable");
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester");
}

fn main() {
    show_info();

    // Parse command line arguments
    // First argument is input .wav file, second argument is output text file.
    let args: Vec<String> = std::env::args().collect();

    // Open the input wave file and determine number of channels
    let mut reader = WavReader::open(&args[1]).unwrap();
    let num_channels = reader.spec().channels;

    // Open the output text file
    let mut out_file = File::create(&args[2]).unwrap();

    // Read audio data and write it to the output text file (one column per channel)
    for sample in reader.samples::<i16>() {
        let sample = sample.unwrap() as f32 / i16::MAX as f32;
        write!(out_file, "{:.6}\n", sample).unwrap();
    }
}