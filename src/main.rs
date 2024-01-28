use std::{env, fs::File, io::Write};
use hound::WavReader;

fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable");
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester");
}

fn main() {
    show_info();

    // Parse command line arguments
    // First argument is input .wav file, second argument is output text file.
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_wav_file> <output_txt_file>", args[0]);
        std::process::exit(1);
    }

    // Open the input wave file and determine the number of channels
    let input_filename = &args[1];
    let output_filename = &args[2];

    let mut reader = WavReader::open(input_filename).expect("Failed to open input wav file");
    let num_channels = reader.spec().channels;

    // Read audio data and write it to the output text file (one column per channel)
    let mut output_file = File::create(output_filename).expect("Failed to create output txt file");

    for sample in reader.samples::<f32>() {
        match sample {
            Ok(channel_sample) => {
                write!(output_file, "{} ", channel_sample).expect("Failed to write to output txt file");
            }
            Err(e) => {
                eprintln!("Error reading sample: {}", e);
                std::process::exit(1);
            }
        }
    }
}
