use crate::ring_buffer::RingBuffer;

pub struct WavetableLfo {
    wavetable: RingBuffer<f32>, //  using RingBuffer to store the wavetable
    phase_increment: f32,
    current_phase: f32,
    amplitude: f32,
    sample_rate: f32,
}

impl WavetableLfo {
    pub fn new(sample_rate: f32, wavetable_size: usize) -> Self {
        let mut wavetable = RingBuffer::new(wavetable_size);

        // Populate the wavetable with a single cycle of a sine wave
        for i in 0..wavetable_size {
            let phase = (i as f32) / (wavetable_size as f32) * 2.0 * std::f32::consts::PI;
            wavetable.push(phase.sin());
        }

        WavetableLfo {
            wavetable,
            phase_increment: 0.0, // This will be set based on the frequency
            current_phase: 0.0,
            amplitude: 1.0, // Default amplitude value
            sample_rate,
        }
    }

    // Sets the LFO frequency
    pub fn set_frequency(&mut self, frequency: f32) {
        // Calculate how much to increment the phase by for each sample
        self.phase_increment = frequency * (self.wavetable.capacity() as f32) / self.sample_rate;
    }

    // Sets the LFO amplitude
    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }

    // Generates the next LFO value
    pub fn next(&mut self) -> f32 {
        // Calculate the current index into the wavetable based on the current phase
        let index = (self.current_phase as usize) % self.wavetable.capacity();
        let value = self.wavetable.get(index) * self.amplitude;

        // Increment the phase for the next sample, wrapping around if necessary
        self.current_phase += self.phase_increment;
        while self.current_phase >= self.wavetable.capacity() as f32 {
            self.current_phase -= self.wavetable.capacity() as f32;
        }

        value
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer module to use in tests

    #[test]
    fn test_wavetable_lfo() {
        // Sample rate and wavetable size for the LFO
        let sample_rate = 44100.0;
        let wavetable_size = 1024;

        // Initialize the LFO
        let mut lfo = WavetableLfo::new(sample_rate, wavetable_size);

        // Set LFO parameters
        lfo.set_frequency(5.0); // Example frequency
        lfo.set_amplitude(0.5); // Example amplitude

        // Variable to track LFO output
        let mut min_value = f32::MAX;
        let mut max_value = f32::MIN;

        // Simulate processing loop and collect LFO output
        for _ in 0..sample_rate as usize {
            let lfo_value = lfo.next();
            min_value = min_value.min(lfo_value);
            max_value = max_value.max(lfo_value);
        }

        // Check if LFO values are within expected range
        // Since we set amplitude to 0.5, values should be between -0.5 and 0.5
        assert!(min_value >= -0.5, "LFO output lower than expected: {}", min_value);
        assert!(max_value <= 0.5, "LFO output higher than expected: {}", max_value);
    }


    #[test]
    fn test_wavetable_lfo_one_phase() {
        let sample_rate = 44100.0;
        let wavetable_size = 44100;
        let frequency = 1.0; // Set frequency to 1 Hz for easy calculation

        // Initialize the WavetableLfo
        let mut lfo = WavetableLfo::new(sample_rate, wavetable_size);
        lfo.set_frequency(frequency); // 1 Hz means one cycle per second

        // Calculate the number of samples to cover one phase (cycle) at the given frequency
        let samples_per_cycle = sample_rate / frequency;

        println!("Wavetable LFO Values for One Phase:");
        for _ in 0..samples_per_cycle as usize {
            let lfo_value = lfo.next();
            println!("{}", lfo_value);
        }

        // No specific assertion here since we're focusing on printing values
        // In a real test scenario, consider adding assertions to verify the correctness of the LFO implementation
    }


}

