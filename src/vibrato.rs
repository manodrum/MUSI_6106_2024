use crate::ring_buffer::RingBuffer;
use crate::lfo::LFO;

pub struct Vibrato {
    buffer: RingBuffer<f32>,
    lfo: LFO,
    delay: f32,
    sample_rate: usize
}

impl Vibrato {
    pub fn new(freq: f32, max_delay_secs: f32, sample_rate: usize) -> Self {
        Vibrato {
            buffer: RingBuffer::new((max_delay_secs * sample_rate as f32) as usize),
            lfo: LFO::new(freq, sample_rate),
            delay: 0.0,
            sample_rate: sample_rate,
        }
    }

    pub fn set_delay(&mut self, delay_in_secs: f32) {
        self.delay = delay_in_secs * self.sample_rate as f32;
    }

    pub fn process_block(&mut self, input: &[f32], output: &mut [f32]) {
        let mut lfo_samples = vec![0.0; input.len()];
        self.lfo.get_block(lfo_samples.as_mut_slice());

        for i in 0..input.len() {
            self.buffer.push(input[i]);
            self.buffer.set_read_index(self.buffer.get_write_index() as i32 - self.delay as i32);
            dbg!(self.buffer.get_read_index());
            dbg!(self.buffer.get_write_index());
            let offset = (lfo_samples[i]) * self.delay;
            output[i] = self.buffer.get_frac(offset);
        }
    }
}