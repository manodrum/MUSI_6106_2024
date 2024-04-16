pub struct FastConvolver<'a> {
    impulse_response: &'a[f32],
    mode: ConvolutionMode,
    remaining_sections: Vec<f32>
}

#[derive(Debug, Clone, Copy)]
pub enum ConvolutionMode {
    TimeDomain,
    FrequencyDomain { block_size: usize },
}

impl<'a> FastConvolver<'a> {
    pub fn new(impulse_response: &'a[f32], mode: ConvolutionMode) -> Self {
        // impulse_response
        Self {
            impulse_response: impulse_response,
            mode: mode,
            remaining_sections: Vec::new()
        }
    }

    pub fn reset(&mut self) {
        todo!("Implement")
    }

    pub fn process(&mut self, input: &[f32], output: &mut [f32]) {
        match self.mode {
            ConvolutionMode::TimeDomain => {
                self.convolve(input, self.impulse_response, output)
            },
            ConvolutionMode::FrequencyDomain { block_size } => {

            }
        }
    }

    pub fn flush(&mut self, output: &mut [f32]) {
        todo!("implement")
    }

    pub fn convolve(&mut self, input1: &[f32], input2: &[f32], output: &mut [f32]) {
        for (idx1, i1) in input1.iter().enumerate() {
            for (idx2, i2) in input2.iter().enumerate() {
                // println!("{}, {}, {}", i1, i2, idx1+idx2);
                output[idx1 + idx2] += i1*i2;
            }
        }
    }
}

// TODO: feel free to define other types (here or in other modules) for your own use
