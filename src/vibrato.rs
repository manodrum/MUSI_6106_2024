use crate::ring_buffer::RingBuffer;
use crate::lfo::LFO;

pub struct Vibrato {
    buffer: RingBuffer<f32>,
    lfo: LFO,
    delay: f32
}

impl Vibrato {
    pub fn new(freq: f32, max_delay_secs: f32, sample_rate: usize) -> Self {
        Vibrato {
            buffer: RingBuffer::new((max_delay_secs * sample_rate as f32) as usize),
            lfo: LFO::new(freq, sample_rate),
            delay: 0.0
        }
    }

    pub fn set_delay(&mut self, delay: f32) {
        self.delay = delay;
    }

    pub fn process_block(&mut self, input: &[f32], output: &mut [f32]) {
        let mut lfo_samples = vec![0.0; output.len()];
        self.lfo.get_block(lfo_samples.as_mut_slice());

        for i in 0..input.len() {
            self.buffer.push(input[i]);
            let offset = lfo_samples[i] * self.delay;
            output[i] = self.buffer.get_frac(offset);
        }
    }
}