use crate::ring_buffer::RingBuffer;
use crate::lfo::WavetableLfo;

pub struct Vibrato {
    buffer: RingBuffer<f32>,
    lfo: WavetableLfo,
    sample_rate_hz: f32,
    depth: f32, // Amplitude of LFO modulation
    freq: f32,  // Frequency of LFO modulation
}

impl Vibrato {
    pub fn new(sample_rate_hz: f32, depth: f32, mod_freq: f32, max_delay_secs: f32) -> Self {
        let buffer_size = (max_delay_secs * sample_rate_hz) as usize;
        let lfo = WavetableLfo::new(sample_rate_hz, 1024); // Example wavetable size

        let mut vibrato = Vibrato {
            buffer: RingBuffer::new(buffer_size),
            lfo,
            sample_rate_hz,
            depth: 0.0, // Initialize with default values
            freq: 0.0,  // Will be set properly using set_param
        };

        // Now using set_param to apply the initial settings
        vibrato.set_param("frequency", mod_freq);
        vibrato.set_param("depth", depth);

        vibrato
    }

    pub fn reset(&mut self) {
        self.buffer.reset();
        // Resetting LFO phase might also be necessary depending on LFO implementation
        // self.lfo.reset_phase();
    }

    pub fn process(&mut self, input_sample: f32) -> f32 {
        let lfo_value = self.lfo.next();
        let delay_amount = self.depth * lfo_value; // Calculate modulation depth
        let delayed_sample = self.buffer.get_frac(delay_amount); // Fetch interpolated sample

        self.buffer.push(input_sample);

        delayed_sample
    }

    pub fn set_param(&mut self, param: &str, value: f32) {
        match param {
            "frequency" => {
                self.freq = value; // Update the freq field
                self.lfo.set_frequency(value); // Update the LFO's frequency
            },
            "depth" => self.depth = value,
            _ => unimplemented!(),
        }
    }

    pub fn get_param(&self, param: &str) -> f32 {
        match param {
            "frequency" => self.freq,
            "depth" => self.depth,
            _ => unimplemented!(),
        }
    }

}

fn main() {
    // Example setup parameters
    let sample_rate_hz = 44100.0; // Standard audio sample rate
    let depth = 0.005; // Vibrato depth in seconds (5 milliseconds)
    let mod_freq = 5.0; // Modulation frequency in Hz
    let max_delay_secs = 0.01; // Maximum delay time for vibrato effect

    // Initialize the Vibrato effect
    let mut vibrato = Vibrato::new(sample_rate_hz, depth, mod_freq, max_delay_secs);

    // Example audio signal: A simple sine wave
    let sine_wave_freq = 440.0; // Frequency of the sine wave (A4 note)
    let duration_secs = 1; // Duration of the audio signal in seconds
    let num_samples = (sample_rate_hz * duration_secs as f32) as usize;

    // Generate a sine wave
    let sine_wave: Vec<f32> = (0..num_samples)
        .map(|i| {
            let t = i as f32 / sample_rate_hz;
            (2.0 * std::f32::consts::PI * sine_wave_freq * t).sin()
        })
        .collect();

    // Apply vibrato effect to the sine wave
    let vibrato_signal: Vec<f32> = sine_wave.iter().map(|&sample| vibrato.process(sample)).collect();

    // Optionally, adjust parameters during processing
    vibrato.set_param("frequency", 6.0); // Change LFO frequency
    vibrato.set_param("depth", 0.007); // Change vibrato depth

    // Continue processing with new settings...
    // For simplicity, this example does not output sound directly but manipulates audio data
}

#[cfg(test)]
mod tests {
    use super::*; // Import the Vibrato struct and any other necessary items from the outer module

    #[test]
    fn vibrato_effect_processing() {
        // Setup parameters similar to the main example
        let sample_rate_hz = 44100.0;
        let depth = 0.005; // Example depth in seconds
        let mod_freq = 5.0; // Example modulation frequency in Hz
        let max_delay_secs = 0.01; // Maximum delay time for the vibrato effect

        // Initialize the Vibrato effect
        let mut vibrato = Vibrato::new(sample_rate_hz, depth, mod_freq, max_delay_secs);

        // Generate a simple test signal: a single cycle of a sine wave
        let sine_wave_freq = 440.0; // Frequency of the sine wave (A4 note)
        let num_samples = sample_rate_hz as usize / sine_wave_freq as usize; // Number of samples for 1 cycle

        let sine_wave: Vec<f32> = (0..num_samples)
            .map(|i| {
                let t = i as f32 / sample_rate_hz;
                (2.0 * std::f32::consts::PI * sine_wave_freq * t).sin()
            })
            .collect();

        // Apply the vibrato effect to the sine wave
        let vibrato_signal: Vec<f32> = sine_wave.iter()
            .map(|&sample| vibrato.process(sample))
            .collect();

        // Assertions
        // For simplicity, this example checks that the process does not produce NaN values
        assert!(vibrato_signal.iter().all(|&sample| sample.is_finite()), "Vibrato processing should not produce non-finite values");

        // Optionally, test parameter adjustments
        vibrato.set_param("frequency", 6.0); // Example of changing LFO frequency
        vibrato.set_param("depth", 0.007); // Example of changing vibrato depth

        // Ensure parameters are set correctly
        assert_eq!(vibrato.get_param("frequency"), 6.0, "LFO frequency should be updated to 6.0");
        assert_eq!(vibrato.get_param("depth"), 0.007, "Vibrato depth should be updated to 0.007");
    }



    #[test]
    fn vibrato_effect_on_unit_signal() {
        // Setup parameters
        let sample_rate_hz = 44100.0;
        let depth = 0.0; // Modulation depth in seconds
        let mod_freq = 500.0; // Modulation frequency in Hz
        let max_delay_secs = 1.0; // Maximum delay time for vibrato effect
        let num_samples = sample_rate_hz as usize * 1; // Process 1 second of audio

        // Initialize the Vibrato effect
        let mut vibrato = Vibrato::new(sample_rate_hz, depth, mod_freq, max_delay_secs);

        // Generate a unit signal (constant value)
        let unit_signal = vec![1.0; num_samples]; // Vector of 1.0s

        // Apply the vibrato effect to the unit signal
        let vibrato_signal: Vec<f32> = unit_signal.iter()
            .map(|&sample| vibrato.process(sample))
            .collect();

        // Print the processed signal
        // Note: In a real test, you might limit the number of samples printed, or compare against expected values
        println!("Processed Signal Samples:");
        vibrato_signal.iter().take(100).enumerate().for_each(|(i, &sample)| {
            println!("Sample {}: {}", i + 1, sample);
        });

        // Assertion placeholder (For actual testing, you should include meaningful assertions here)
        assert!(vibrato_signal.iter().all(|&sample| sample.is_finite()), "Vibrato processing should not produce non-finite values");
    }
}


