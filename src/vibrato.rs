use crate::ring_buffer::RingBuffer;
use crate::lfo::WaveTableLfo;


/// The `Vibrato` struct applies a vibrato effect to audio signals.
/// Vibrato is an effect where the pitch of the sound varies in a periodic manner,
/// creating a slight oscillation that is pleasing to the ear in musical contexts.
///
/// This implementation uses a ring buffer for delay and a wavetable low-frequency oscillator (LFO)
/// for modulation. The delay time is modulated by the LFO to achieve the vibrato effect.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let mut vibrato = Vibrato::new(44100, 0.005, 5.0, 0.02, 2);
/// // Assuming `input` is a buffer with your input signal and `output` is where you want to store the processed signal.
/// vibrato.process(&[&input], &mut [&mut output]);
/// ```
///
/// # External Resources
///
/// For more information on the vibrato effect and its musical applications, see [Wikipedia](https://en.wikipedia.org/wiki/Vibrato).
///
///
///
///
///
/*
The `Vibrato` struct is designed to apply a vibrato effect to a slice of a slice (audio slices from number of channels).
The LFO value is obtained as a vector once and used for all channels. The phase is updated when all slices are processed.

1. **Use of Ring Buffer for Delay**:
   - I choose to set the size of lfo table to sample rate, and use get frac to get any value in between indexs, call the lfo with sample index of the processed audio and return the modulated width in samples
   - The `RingBuffer` is employed to create a variable delay in the audio signal, which is essential for the vibrato effect. The ring buffer's size is determined by the maximum delay time (`max_delay_secs`) and the sample rate, allowing for efficient memory usage while accommodating the necessary delay length.
   - This design choice enables the `Vibrato` to simulate the natural fluctuation in pitch characteristic of vibrato by modulating the delay time.

2. **WaveTable LFO for Modulation**:
   - The `WaveTableLfo` is used to modulate the delay time of the audio signal, which alters the pitch and creates the vibrato effect. The LFO's frequency (`mod_freq`) and amplitude (`depth`) are adjustable, offering control over the rate and intensity of the vibrato.
   - Wavetable synthesis provides an efficient way to generate smooth, periodic waveforms for modulation. This choice ensures the LFO can accurately and efficiently produce a consistent modulation effect over time.

3. **Parameter Adjustment through `set_param` Method**:
   - The `set_param` method allows for dynamic adjustment of the vibrato's modulation frequency (`mod_freq`) and depth (`depth`). This flexibility lets users customize the vibrato effect to their preferences or requirements dynamically.
   - Using a string to identify parameters provides an easy-to-use interface for adjusting parameters, though it trades off type safety for flexibility.


 */

pub struct Vibrato {
    buffer: Vec<RingBuffer<f32>>,
    lfo: WaveTableLfo,
    sample_rate_hz: usize,
    depth: f32, // Amplitude of LFO modulation as in seconds
    mod_freq: f32,  // Frequency of LFO modulation
    delay: f32
}

enum ParamValue {
    Frequency(usize),
    Amplitude(f32),
}

fn process_mono(delay_line: &mut RingBuffer<f32>, input: &[f32], output: &mut [f32], delay: f32, lfo_list: &[f32]) {


    for i in 0.. input.len(){
        delay_line.set_read_index(delay_line.get_write_index() as i32 - delay as i32);
        // println!("write_index: {}", self.buffer.get_write_index());
        delay_line.push(input[i]);
        // println!("read_index: {}", self.buffer.get_read_index());
        output[i] = delay_line.get_frac(lfo_list[i]);

        // println!("lfo value: {}", lfo_list[i]);
        // println!("output[{}]: {}", i, self.buffer.get_frac(self.lfo.get_value()));
    }
}


impl Vibrato {
    pub fn new(sample_rate_hz: usize, depth_in_sec: f32, mod_freq: f32, delay_time: f32, num_channels: usize) -> Self {

        if delay_time < depth_in_sec {
            eprintln!("Warning: The delay time ({}) is smaller than the depth in seconds ({}). This may lead to unintended behavior.", delay_time, depth_in_sec);
        }

        let mut buffer = Vec::with_capacity(num_channels);
        let delay_line_size = (delay_time * sample_rate_hz as f32).ceil() as usize + 1;
        for _ in 0..num_channels {
            let delay_line = RingBuffer::new(delay_line_size);
            buffer.push(delay_line);
        };

        Vibrato {
            buffer,
            lfo : WaveTableLfo::new(sample_rate_hz, mod_freq, depth_in_sec), // Example wavetable size,
            sample_rate_hz,
            depth: depth_in_sec, // Initialize with default values
            mod_freq: mod_freq,  // Will be set properly using set_param
            delay: delay_time,
        }

    }

    pub fn reset(&mut self) {
        for buffer in &mut self.buffer {
            buffer.reset()
        }
    }


    pub fn process(&mut self, input: &[&[f32]], output: &mut [&mut [f32]]) {
        // Process each input/output channel using the corresponding delay line.

        let mut delay = self.depth * self.sample_rate_hz as f32;

        // Create a vector to hold LFO values for each sample.
        let mut lfo_values = Vec::with_capacity(self.buffer.len());
        for i in 0..input[0].len() {
            lfo_values.push(self.lfo.get_value_at_sample_index(i));
            // println!("lfo value: {}", self.lfo.get_value_at_sample_index(i));
        }
        // Obtain a slice for read-only use from the vector.
        let lfo_list: &[f32] = &lfo_values;

        for i in 0..self.buffer.len() {
            process_mono(&mut self.buffer[i], input[i], output[i], delay, lfo_list);
        }

        self.lfo.update_phase(input[0].len() as f32);
    }



    pub fn set_param(&mut self, param: &str, value: f32) {
        match param {              // switch case if param = frequency then set ... else ...set
            "frequency" => {

                self.mod_freq = value; // Update the freq field
                self.lfo.set_value("mod_freq", value); // Update the LFO's frequency
            },
            "depth" => {
                if self.delay < value {
                    eprintln!("Warning: The delay time ({}) is smaller than the depth in seconds ({}). This may lead to unintended behavior.", self.delay, value);
                }

                self.lfo.set_value("amplitude", value);
                self.depth = value;

            },
            _ => unimplemented!(),
        }
    }

    pub fn get_param(&self, param: &str) -> f32 {
        match param {
            "frequency" => self.mod_freq,
            "depth" => self.depth,
            _ => unimplemented!(),
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*; // Import the Vibrato struct and any other necessary items from the outer module
    use approx::assert_relative_eq;


    #[test]
    fn test_output_equals_delayed_input_with_zero_depth() {
        let sample_rate_hz = 48000;
        let depth_in_sec = 0.0; // No modulation depth
        let mod_freq = 1.0; // Arbitrary, won't affect the test due to 0 depth
        let delay_time_sec = 0.01; // 10ms delay
        let num_channels = 1; // Mono signal for simplicity
        let input_len = 100; // Length of the test input signal

        let mut vibrato = Vibrato::new(sample_rate_hz, depth_in_sec, mod_freq, delay_time_sec, num_channels);

        let input_signal: Vec<f32> = (0..input_len).map(|x| x as f32).collect();
        let mut output_signal: Vec<f32> = vec![0.0; input_len];

        vibrato.process(&[&input_signal], &mut [&mut output_signal]);

        // Calculate the delay in samples
        let delay_samples = (delay_time_sec * sample_rate_hz as f32).round() as usize;

        // Iterate through the output signal to verify each sample
        for i in 0..input_len {
            if i < delay_samples {
                // // For initial delay samples, the output should be 0 (or initial state of the buffer)
                // assert!(output_signal[i] == 0.0);
            } else {
                // Calculate the difference between the output and the delayed input
                let diff = (output_signal[i] - input_signal[i - delay_samples]).abs();
                let epsilon = 1e-5; // Define an acceptable error margin

                // Assert that the difference is smaller than epsilon
                assert!(diff < epsilon, "Difference at index {}: {} is not less than epsilon {}", i, diff, epsilon);
            }
        }
    }


    #[test]
    fn test_dc_input_results_in_dc_output_considering_delay() {
        let sample_rate_hz = 48000;
        let depth_in_sec = 0.1; // Arbitrary depth
        let mod_freq = 5.0; // Arbitrary modulation frequency
        let delay_time_sec = 0.01; // Specific delay time
        let num_channels = 1; // Mono signal
        let input_len = 500; // Length of the test input signal to ensure delay is covered
        let dc_level = 1.0; // DC level for the input signal

        let mut vibrato = Vibrato::new(sample_rate_hz, depth_in_sec, mod_freq, delay_time_sec, num_channels);

        // Create a DC input signal
        let input_signal: Vec<f32> = vec![dc_level; input_len];

        // Placeholder for the output signal
        let mut output_signal: Vec<f32> = vec![0.0; input_len];

        // Process the input signal through Vibrato
        vibrato.process(&[&input_signal], &mut [&mut output_signal]);

        // Calculate the delay in samples
        let delay_samples = (delay_time_sec * sample_rate_hz as f32).ceil() as usize;

        // Verify that after the initial delay, the output signal matches the DC level
        let epsilon = 1e-5;
        for i in delay_samples..input_len {
            assert!(
                (output_signal[i] - dc_level).abs() < epsilon,
                "Output sample at index {} : {} does not match DC level: {} within epsilon: {}",
                i, output_signal[i], dc_level, epsilon
            );
        }
    }

    #[test]
    fn test_vibrato_with_varying_input_block_sizes() {
        let sample_rate_hz = 48000;
        let depth_in_sec = 0.05; // Arbitrary depth
        let mod_freq = 2.0; // Arbitrary modulation frequency
        let delay_time_sec = 0.01; // Arbitrary delay time
        let num_channels = 1; // Mono signal

        let input_signal_len = 48000; // 1 second of audio at 48kHz
        let input_signal: Vec<f32> = (0..input_signal_len).map(|x| (x as f32 * 2.0 * std::f32::consts::PI / sample_rate_hz as f32).sin()).collect();

        let mut vibrato = Vibrato::new(sample_rate_hz, depth_in_sec, mod_freq, delay_time_sec, num_channels);

        // Define varying block sizes
        let block_sizes = vec![64, 128, 256, 512, 1024];

        let mut output_signal = Vec::new();
        let mut start_index = 0;

        // Process each block
        for &block_size in &block_sizes {
            let end_index = start_index + block_size;
            if end_index > input_signal_len { break; }

            let input_block = &input_signal[start_index..end_index];
            let mut output_block = vec![0.0; block_size];

            vibrato.process(&[input_block], &mut [&mut output_block[..]]);
            output_signal.extend_from_slice(&output_block);

            start_index = end_index;
        }

        assert_eq!(output_signal.len(), start_index, "Processed output length does not match expected length.");
    }


    #[test]
    fn test_zero_input_results_in_zero_output() {
        let sample_rate_hz = 48000;
        let depth_in_sec = 0.05; // Arbitrary, but non-zero depth
        let mod_freq = 2.0; // Arbitrary modulation frequency
        let delay_time_sec = 0.01; // Arbitrary delay time, to test delay handling
        let num_channels = 1; // Mono signal
        let input_len = 1024; // Arbitrary length of the input signal

        let mut vibrato = Vibrato::new(sample_rate_hz, depth_in_sec, mod_freq, delay_time_sec, num_channels);

        // Create a zero input signal
        let input_signal = vec![0.0f32; input_len];
        let mut output_signal = vec![0.0f32; input_len];

        // Process the zero input signal through Vibrato
        vibrato.process(&[&input_signal], &mut [&mut output_signal]);

        // Verify that the output signal is also zero
        let epsilon = 1e-6; // Define a small epsilon for floating-point comparisons
        for &sample in &output_signal {
            assert!((sample - 0.0).abs() < epsilon, "Non-zero output sample detected: {}", sample);
        }
    }

    #[test]
    fn test_vibrato_initialization() {
        let sample_rate_hz = 44100;
        let depth = 0.5;
        let mod_freq = 5.0;
        let mut vibrato = Vibrato::new(sample_rate_hz, depth, mod_freq, 4.0, 1);

        assert_eq!(vibrato.sample_rate_hz, sample_rate_hz);
        assert_eq!(vibrato.depth, 0.5); // Since depth is initialized to 0.0
        assert_eq!(vibrato.mod_freq, 5.0); // Since mod_freq is initialized to 0.0
    }

    #[test]
    fn test_set_and_get_params() {
        let mut vibrato = Vibrato::new(44100, 0.5, 5.0, 4.0,1);

        println!("{}", vibrato.buffer.len());

        vibrato.set_param("frequency", 10.0);
        assert_eq!(vibrato.get_param("frequency"), 10.0);

        vibrato.set_param("depth", 7.0);
        assert_eq!(vibrato.get_param("depth"), 7.0);
    }


}

