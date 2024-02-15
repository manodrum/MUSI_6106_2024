// extern crate hound;

use std::{fs::File, io::Write, os::unix::process, process::exit, ptr::null, result, sync::mpsc::channel};
mod comb_filter;
use comb_filter::{CombFilter, FilterParam, FilterType};
// use hound::{WavWriter, WavReader};

fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable");
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester");
}
const max_block_size : usize = 5000;
const max_data_length : usize = 1000000;
const max_channel_length : usize = 5;
static mut data_array : [[f32; max_data_length]; max_channel_length] 
            = [[0.0; max_data_length]; max_channel_length];
fn main() {
   show_info();
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    //TODO: run tests when there are no args given 
    if args.len() == 1
    {
        println!("Running tests...");   
        
        run_test_3();
        run_test_4();
        run_test_5();
        return;
    }


    if args.len() < 6 {
        eprintln!("Usage: {} <input wave filename> <output text filename> <FIR/IIR> <delay value> <gain value>", args[0]);
        return
    }
    
    let number_str1 = &args[4];
    let delay: f32 = number_str1.parse().unwrap();
    let number_str2 = &args[5];
    let gain: f32 = number_str2.parse().unwrap();
    let mut comb_filter = CombFilter::new (FilterType::FIR,0.1,44100.0,2);
    if args[3] == "IIR"
    {
        println!("Using IIR Filter");
        comb_filter = CombFilter::new (FilterType::IIR,0.1,44100.0,2);
    }
    // comb_filter.set_param(FilterParam::Delay, 0.5).unwrap();
    // comb_filter.set_param(FilterParam::Gain, 2.5).unwrap();
    // let delay_value = comb_filter.get_param(FilterParam::Delay);
    // println!("original params:");
    // println!("Delay parameter: {}", delay_value);
    // let gain_value = comb_filter.get_param(FilterParam::Gain);
    // println!("Gain parameter: {}", gain_value);
    // comb_filter.reset();
    // let delay_value = comb_filter.get_param(FilterParam::Delay);
    // println!("after reset:");
    // println!("Delay parameter: {}", delay_value);
    // let gain_value = comb_filter.get_param(FilterParam::Gain);
    // println!("Gain parameter: {}", gain_value);

   

    comb_filter.set_param(FilterParam::Delay, delay).unwrap();
    comb_filter.set_param(FilterParam::Gain, gain).unwrap();
    let delay_value = comb_filter.get_param(FilterParam::Delay);
    println!("from args:");
    println!("Delay parameter: {}", delay_value);
    let gain_value = comb_filter.get_param(FilterParam::Gain);
    println!("Gain parameter: {}", gain_value);


    // // Open the input wave file
    let mut reader = hound::WavReader::open(&args[1]).unwrap();
    let spec = reader.spec();
    // let channels = spec.channels;

    // Get WAV file properties
    let channels = reader.spec().channels as usize;
    let sample_rate = reader.spec().sample_rate as usize;
    println!("{}, {}", channels, sample_rate);
    
    
    // TODO: Modify this to process audio in blocks using your comb filter and write the result to an audio file.
    //       Use the following block size:
    let block_size : usize = 1024;
    let mut pos = 0;
    let mut data_length = 0;
    for (i, sample) in reader.samples::<i16>().enumerate() {
        let sample = sample.unwrap() as f32 / (1 << 15) as f32;
        let cur_channel = i % channels;
        unsafe{
        data_array[cur_channel][pos] = sample;
        }
        if cur_channel == channels - 1
        {
            pos = pos + 1; 
            data_length = pos;
        }
           
    }

    process_data_array(block_size, channels, &mut comb_filter, data_length);




    
    // Read audio data and write it to the output text file (one column per channel)
    let mut out = File::create(&args[2]).expect("Unable to create file");
    for cur_data_index in 0..data_length
    {
        for cur_channel_index in 0..channels
        {
            let mut cur_data = 0.0;
            unsafe
            {
                cur_data = data_array[cur_channel_index][cur_data_index];
            }
            if cur_channel_index == channels - 1
            {
                writeln!(out, "{}", cur_data).unwrap();
            }
            else 
            {
                write!(out, "{} ", cur_data).unwrap();
            }
        }
    }
    
    
}
fn process_data_array(block_size : usize, channels : usize, comb_filter:&mut CombFilter, data_length:usize)
{
    
    for cur_channel in 0..channels
    {
        let mut cur_block = 0;
        loop {
            let mut finished = false;
            let start_index = cur_block * block_size;
            let mut block_data : [f32; max_block_size] = [0.0; max_block_size];
            for data_index in 0 .. block_size
            {
                let cur_index = start_index + data_index;
                unsafe{
                    if cur_index < data_length
                    {
                        block_data[data_index] = data_array[cur_channel][cur_index];
                    }
                    else 
                    {
                        finished = true;
                    }
                }
            } 
            let mut processed_data : [f32; max_block_size] = [0.0; max_block_size];
            let mut block_data_single_channel = [& block_data[0..block_size]];
            let mut processed_data_single_channel = [&mut processed_data[0..block_size]];
            //comb_filter.process_block(&mut block_data, &mut processed_data, block_size);
            comb_filter.process(& block_data_single_channel[0..1], &mut processed_data_single_channel[0..1]);
            for data_index in 0 .. block_size
            {
                let cur_index = start_index + data_index;
                unsafe{
                    if cur_index < data_length
                    {
                        data_array[cur_channel][cur_index] = processed_data[data_index];
                    }
                    else 
                    {
                        break;
                    }
                }
            } 
            if finished
            {
                break;
            }
            cur_block = cur_block + 1;
            
        }

    }
}
fn generate_sine_wave(num_samples: usize, frequency: f32, amplitude: f32, sample_rate: f32)  
{
    use std::f32::consts::PI;
    for cur_index in 0..num_samples
    {
        let cur_number = amplitude * (2.0 * PI * frequency * cur_index as f32 / sample_rate).sin();
        unsafe
        {
            data_array[0][cur_index] = cur_number;
        }
    }
    
}
fn run_test_1()
{
    generate_sine_wave(200000, 10000.0, 1.0, 50000.0);
    let mut comb_filter = CombFilter::new(FilterType::FIR, 0.0025, 50000.0, 1);
    comb_filter.set_param(FilterParam::Gain, 1.0).unwrap();
    comb_filter.set_param(FilterParam::Delay, 0.0025).unwrap();
    
    process_data_array(1000, 1, &mut comb_filter, 20000);
    for cur_channel in 0..2
    {
        for cur_index in 0..5 
        {
            unsafe
            {
                print!("{} ", data_array[cur_channel][cur_index]);
            }
        }
        println!("");
    }
}
fn run_test_3()
{
    for i in 0..max_data_length
    {
        unsafe
        {
            data_array[0][i] = i as f32 / max_data_length as f32;
        }
    }

    let mut comb_filter = CombFilter::new(FilterType::FIR, 0.0025, 50000.0, 1);
    comb_filter.set_param(FilterParam::Gain, 1.0).unwrap();
    comb_filter.set_param(FilterParam::Delay, 0.0025).unwrap();
    
    process_data_array(1000, 1, &mut comb_filter, 20000);
    let mut result_1 : [f32; 20000] = [0.0; 20000];
    for i in 0..20000
    {
        unsafe
        {
            result_1[i] = data_array[0][i];
        }
    }


    for i in 0..max_data_length
    {
        unsafe
        {
            data_array[0][i] = i as f32 / max_data_length as f32;
        }
    }
    process_data_array(2000, 1, &mut comb_filter, 20000);
    for i in 0..20000
    {
        unsafe{

        assert!(result_1[i] == data_array[0][i]);
        }
    }
    println!("Test 3 passed");
}

fn run_test_4()
{
    for i in 0..max_data_length
    {
        unsafe
        {
            data_array[0][i] = 0.0;
        }
    }
    let mut comb_filter = CombFilter::new(FilterType::FIR, 0.0025, 50000.0, 1);
    comb_filter.set_param(FilterParam::Gain, 1.0).unwrap();
    comb_filter.set_param(FilterParam::Delay, 0.0025).unwrap();
    
    process_data_array(1000, 1, &mut comb_filter, 20000);
    
    for cur_index in 0..max_data_length
    {
        unsafe
        {
            assert!(data_array[0][cur_index] == 0.0);
        }
    }
    println!("Test 4 passed");
    
}
fn run_test_5()
{
    println!("Test 5 : check if the results match when using different data length");
    println!("Assume the input signal is the impulse");
    for i in 0..max_data_length
    {
        unsafe
        {
            data_array[0][i] = 0.0;
        }
    }
    unsafe{
    data_array[0][0] = 1.0;
    }
    let mut comb_filter = CombFilter::new(FilterType::FIR, 0.0025, 50000.0, 1);
    comb_filter.set_param(FilterParam::Gain, 1.0).unwrap();
    comb_filter.set_param(FilterParam::Delay, 0.0025).unwrap();
    
    process_data_array(1000, 1, &mut comb_filter, 20000);
    let mut result_1 : [f32; 20000] = [0.0; 20000];
    for i in 0..20000
    {
        unsafe
        {
            result_1[i] = data_array[0][i];
        }
    }


    for i in 0..max_data_length
    {
        unsafe
        {
            data_array[0][i] = 0.0;
        }
    }
    unsafe{
    data_array[0][0] = 1.0;
    }
    process_data_array(2000, 1, &mut comb_filter, 10000);
    for i in 0..10000
    {
        unsafe{

        assert!(result_1[i] == data_array[0][i]);
        }
    }
    println!("Test 5 passed");
}
