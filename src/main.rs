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
  let mut audio_in  = WavReader::open(&args[1]).unwrap();
  let num_channels = audio_in.spec().channels as usize;

  // Open the output text file
  let mut out_text = File::create(&args[2]).unwrap();

  // Read audio data and write it to the output file (one column per channel)
  for (i, sample) in audio_in.samples::<i16>().enumerate() {
    let sample = sample.unwrap() as f32 / i16::MAX as f32;
    if i % num_channels == 0 {
      write!(out_text, "{:.6} ", sample).unwrap();
    } else {
      write!(out_text, "{:.6}\n", sample).unwrap(); 
    }
  }
}