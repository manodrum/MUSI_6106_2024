use crate::ring_buffer::RingBuffer;

pub struct WaveTableLfo {
    wave_table: RingBuffer<f32>, //  using RingBuffer to store the wavetable
    mod_freq: usize,
    amplitude: f32,
    sample_freq: usize,

}

impl WaveTableLfo {
    pub fn new(sample_freq: usize, mod_freq: usize, amplitude: f32) -> Self {

        let mut wave_table_size = sample_freq;
        let mut wave_table = RingBuffer::new(wave_table_size);

        // Populate the wavetable with a single cycle of a sine wave
        for i in 0..wave_table_size {
            let phase =  (i / wave_table_size)  as f32  * 2.0 * std::f32::consts::PI;
            wave_table.push(phase.sin());
        }

        WaveTableLfo {
            wave_table,
            mod_freq,
            amplitude,
            sample_freq,

        }
    }
    pub fn get_value_at_index(&self, index: usize) -> f32 {
        self.amplitude * self.wave_table.get(index)
    }

    pub fn set_value(&mut self, param: &str, value: f32) {
        match param {
            "mod_freq" =>{
                self.mod_freq = value as usize;
            },
            "amplitude" => {
                self.amplitude = value;
            },
            _ => unimplemented!(),
        }
    }



}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_implementation(){
        let size = 44100;
        let mod_freq = 1;
        let sample_rate = 44100;
        let amplitude = 1.0 as f32 ;
        let sine_table = WaveTableLfo::new(sample_rate,mod_freq, amplitude);
        for i in 0..size as usize{
            println!("{}", i);
            println!("{}", sine_table.get_value_at_index(i));
        }
    }

}