use crate::ring_buffer::RingBuffer;

pub struct WaveTableLfo {
    wave_table: RingBuffer<f32>, //  using RingBuffer to store the wavetable
    mod_freq: f32,
    amplitude: f32,
    sample_freq: usize,
    phase: f32,
}

impl WaveTableLfo {
    pub fn new(sample_freq: usize, mod_freq: f32, amplitude: f32) -> Self {

        let mut wave_table_size = sample_freq;
        let mut wave_table = RingBuffer::new(wave_table_size);
        let mut phase = 0.0;

        // Populate the wavetable with a single cycle of a sine wave
        for i in 0..wave_table_size {
            let phase_in_period = (i as f32/ wave_table_size as f32)  * 2.0 * std::f32::consts::PI;
            wave_table.push(phase_in_period.sin());
        }

        WaveTableLfo {
            wave_table,
            mod_freq,
            amplitude: amplitude * sample_freq as f32,
            sample_freq,
            phase,

        }
    }
    pub fn get_value_at_sample_index(&self, index: usize) -> f32 {

        self.amplitude * self.wave_table.get_frac((index as f32 + self.phase) * self.mod_freq as f32)
    }

    pub fn get_value(&self) -> f32 {
        self.amplitude * self.wave_table.get_frac(self.phase as f32 * self.mod_freq as f32)
    }

    pub fn update_phase(&mut self, processed_samples: f32) {
        self.phase =self.phase + processed_samples;
    }

    pub fn set_value(&mut self, param: &str, value: f32) {
        match param {
            "mod_freq" =>{
                self.mod_freq = value;
            },
            "amplitude" => {
                self.amplitude = value * self.sample_freq as f32;
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
        let size = 20;
        let mod_freq = 1.0;
        let sample_rate = 10;
        let amplitude = 2.0 as f32 ;

        let mut sine_table = WaveTableLfo::new(sample_rate,mod_freq, amplitude);


        for i in 44100+ 0..44100+size+1 as usize{
            println!("{}", i);
            println!("{}", sine_table.get_value());
            println!("{}", sine_table.get_value_at_sample_index(i));
            sine_table.update_phase(1.0);
        }
        assert_eq!(7.0, 7.0);
    }

}