use std::{fs::File, io::Write};
use hound;

fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable");
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester");
}

fn main() {
   show_info();

    // Parse command line arguments

    let args: Vec<String> = std::env::args().collect();
    // First argument is input .wav file, second argument is output .txt file
    if args.len() != 3 {
        eprintln!("Usage: {} <input.wav> <output.txt>", args[0]);
        std::process::exit(1);
    }
    let input_filename = &args[1];
    dbg!(input_filename);
    let output_filename = &args[2];
    dbg!(output_filename);
    // Open the input wave file and determine number of channels
    // use `hound::WavReader::open` to open the file
    // use `hound::WavSpec::channels` to get the number of channels
    let mut reader = match hound::WavReader::open(input_filename) {
        Ok(reader) => reader,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            std::process::exit(1);
        }
    };
    let channels = reader.spec().channels;
    let spec = reader.spec();
    eprintln!("Sample rate: {}", spec.sample_rate);
    eprintln!("Channels: {}", spec.channels);
    eprintln!("Sample format: {:?}", spec.sample_format);
    eprintln!("Bits per sample: {}", spec.bits_per_sample);
    // Read audio data and write it to the output text file (one column per channel)
    let mut out_f = File::create(output_filename).unwrap();
    //iterate over channels in the file

for (i, sample) in reader.samples::<i16>().enumerate() {
    let sample = match sample {
        Ok(sample) => sample as f32 / 32768.0,
        Err(e) => {
            eprintln!("Error reading samples: {}", e);
            break;
        }
    };
    write!(out_f, "{}", sample).unwrap();

    // Change here: Adding 1 to `i` in the condition
    if (i + 1) % channels as usize == 0 {
        write!(out_f, "\n").unwrap();
    } else {
        write!(out_f, " ").unwrap();
    }
}

    // TODO: your code here; we suggest using `hound::WavReader::samples`, `File::create`, and `write!`.
    //       Remember to convert the samples to floating point values and respect the number of channels!
}
