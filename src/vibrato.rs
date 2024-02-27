use crate::ring_buffer::RingBuffer;
use crate::lfo::LFO;

pub struct Vibrato {
    buffer: RingBuffer<f32>,
    lfo: LFO
}

impl Vibrato {
    pub fn new(freq: f32, max_delay_secs: f32, sample_rate: usize) -> Self {
        Vibrato {
            buffer: RingBuffer::new((max_delay_secs * sample_rate as f32) as usize),
            lfo: LFO::new(freq, sample_rate)
        }
    }
}