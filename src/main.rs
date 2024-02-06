use std::{fs::File, io::Write};
use crate::ring_buffer::RingBuffer;
mod ring_buffer;

fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable");
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester");
}





fn main() {
   show_info();
    // exercise 1
    // // Parse command line arguments
    // let args: Vec<String> = std::env::args().collect();
    // if args.len() < 3 {
    //     eprintln!("Usage: {} <input wave filename> <output text filename>", args[0]);
    //     return
    // }

    // // Open the input wave file
    // let mut reader = hound::WavReader::open(&args[1]).unwrap();
    // let spec = reader.spec();
    // let channels = spec.channels;

    // // Read audio data and write it to the output text file (one column per channel)
    // let mut out = File::create(&args[2]).expect("Unable to create file");
    // for (i, sample) in reader.samples::<i16>().enumerate() {
    //     let sample = sample.unwrap() as f32 / (1 << 15) as f32;
    //     write!(out, "{}{}", sample, if i % channels as usize == (channels - 1).into() { "\n" } else { " " }).unwrap();
    // }
    
    // exercise 2
    let mut buffer = RingBuffer::new(10);
    
    // push two elements
    buffer.push(42);
    buffer.push(99);
    buffer.display_buffer(|item| format!("{}", item));

    // pop an element
    buffer.pop();
    buffer.display_buffer(|item| format!("{}", item));


    // Append an element to the buffer
    buffer.put(55);
    buffer.display_buffer(|item| format!("{}", item));

    // Peek at the element at the current read index
    if let Some(value) = buffer.peek() {
        println!("Peeked value at the current read index: {:?}", value);
    } else {
        println!("Buffer is empty, cannot peek.");
    }
    buffer.display_buffer(|item| format!("{}", item));

    // Get the element at a specific offset without removing it
    if let Some(value) = buffer.get(1) {
        println!("Got value at offset 1: {:?}", value);
    } else {
        println!("Index out of bounds, cannot get.");
    }
    buffer.display_buffer(|item| format!("{}", item));

    // Reset the buffer
    buffer.reset();
    buffer.display_buffer(|item| format!("{}", item));

    println!("Length: {}", buffer.len());
    println!("Capacity: {}", buffer.capacity());
    println!("Get read index: {}", buffer.get_read_index());
    buffer.set_read_index(5);
    println!("Get read index after set: {}", buffer.get_read_index());
    println!("Get write index: {}", buffer.get_write_index());
    buffer.set_write_index(4);
    println!("Get write index after set: {}", buffer.get_write_index());
    


    
}
