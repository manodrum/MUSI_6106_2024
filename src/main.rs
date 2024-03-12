use std::{fs::File, io::Write};

use vibrato::Vibrato;

mod ring_buffer;
mod vibrato;
mod lfo;

fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable");
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester & Jiahe Qian");
}

fn main() {
    show_info();

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input wave filename> <output wave filename> <vibrato depth in seconds> <modulation frequency>", args[0]);
        return
    }

    // Open the input wave file
    let mut reader = match hound::WavReader::open(&args[1]) {
        Ok(val) => val,
        Err(err) => panic!("Input file not found")
    };
    let spec = reader.spec();
    let channels = spec.channels as usize;
    let block_size = 1024;


    let out = File::create(&args[2]).expect("Unable to create file");
    let mut writer = hound::WavWriter::new(out, spec).unwrap();



    // set up vibrato effect
    let vibrato_depth = args[3].parse().unwrap();
    let vibrato_modulation_frequency = args[4].parse().unwrap();

    //     pub fn new(sample_rate_hz: usize, depth_in_sec: f32, mod_freq: f32, delay_time: f32, num_channels: usize) -> Self {
    let mut vibrato = Vibrato::new(spec.sample_rate as usize, vibrato_depth, vibrato_modulation_frequency, 4.0, channels);
    vibrato.set_param("depth", vibrato_depth);
    vibrato.set_param("frequency", vibrato_modulation_frequency);


    // Read audio data and write it to the output text file (one column per channel)
    let mut input_blocks = vec![Vec::<f32>::with_capacity(block_size); channels];
    let mut output_blocks = vec![vec![0.0_f32; block_size]; channels];
    let num_samples = reader.len() as usize;
    for (i, sample) in reader.samples::<i16>().enumerate() {
        let sample_f32 = sample.unwrap() as f32 / (1 << 15) as f32; // Convert i16 sample to f32
        input_blocks[i % channels].push(sample_f32);
        if (i % (channels * 1024) == 0) || (i == num_samples - 1) {
            // Process block
            let input_slices = input_blocks.iter().map(|channel| channel.as_slice()).collect::<Vec<&[f32]>>();
            let mut output_slices = output_blocks.iter_mut().map(|channel| channel.as_mut_slice()).collect::<Vec<&mut [f32]>>();
            vibrato.process(input_slices.as_slice(), output_slices.as_mut_slice());
            for j in 0..(channels * input_blocks[0].len()) {
                writer.write_sample((output_blocks[j % channels][j / channels] * (1 << 15) as f32) as i32).unwrap();
            }
            for channel in input_blocks.iter_mut() {
                channel.clear();
            }
        }
    }
}