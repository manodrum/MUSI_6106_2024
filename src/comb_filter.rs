// use core::num;


pub struct CombFilter {
    // TODO: your code here
    filter_type:FilterType,
    filter_params:(f32,f32),//gain, delay
    max_delay_secs: f32,
    sample_rate_hz:f32,
    num_channels:usize,
    delay_line: Vec<f32>,
}

#[derive(Debug, Clone, Copy)]
pub enum FilterType {
    FIR,
    IIR,
}

#[derive(Debug, Clone, Copy)]
pub enum FilterParam {
    Gain,
    Delay,
}

#[derive(Debug, Clone)]
pub enum Error {
    InvalidValue { param: FilterParam, value: f32 }
}



impl CombFilter {
    pub fn new(filter_type: FilterType, max_delay_secs: f32, sample_rate_hz: f32, num_channels: usize) -> Self {
        let filter_params = (1.0, 0.0);
        let delay_time = (max_delay_secs * sample_rate_hz) as usize;
        let delay_line = vec![0.0; delay_time];

        return CombFilter { filter_type, filter_params,max_delay_secs,sample_rate_hz, num_channels,delay_line };
    }
    

    pub fn reset(&mut self) {
        self.filter_type = FilterType::FIR; // Replace with the appropriate default value
        self.filter_params= (1.0,0.0); // Replace with the appropriate default value
        self.max_delay_secs= 0.0; // Replace with the appropriate default value
        self.sample_rate_hz = 0.0; // Replace with the appropriate default value
        self.num_channels = 0; // Replace with the appropriate default value
        for x in &mut self.delay_line {
            *x = 0.0;
        }
    }

    pub fn process(&mut self, input: & [& [f32]], output: &mut [&mut [f32]]) {
        let num_channels = input.len();
        for i in 0..num_channels
        {
            self.process_block(input[i], output[i], input[i].len())
        }
    }

    pub fn process_block(&mut self, input: & [f32], output: &mut [f32], block_size : usize) {
        let gain = self.get_param(FilterParam::Gain);
        // let delay= self.get_param(FilterParam::Delay);
        // let mut delay_line = vec![0.0; 10]; // Memory allocation for length 10
    

        match self.filter_type {
            FilterType::FIR => {
                for i in 0..block_size {
                    output[i] = input[i] + gain * self.delay_line[9];
                    // Update delay line by shifting elements to the right
                    self.delay_line.pop();
                    self.delay_line.insert(0, input[i]);
                }
            }
            FilterType::IIR => {
                for i in 0..block_size {
                    output[i] = input[i] + gain * self.delay_line[9];
                    // Update delay line by shifting elements to the right
                    self.delay_line.pop();
                    self.delay_line.insert(0, output[i]);
                }
            }
        }
        
    }    


    pub fn set_param(&mut self, param: FilterParam, value: f32) -> Result<(), Error> {
        match param {
            FilterParam::Delay => {
                self.filter_params.1 = value;
                Ok(())
            }
            FilterParam::Gain => {
                self.filter_params.0 = value;
                Ok(())
            }
        }
        .map_err(|_error: ()| Error::InvalidValue { param, value })
    }

    pub fn get_param(&self, param: FilterParam) -> f32 {
        match param {
            FilterParam::Delay => self.filter_params.1,
            FilterParam::Gain => self.filter_params.0,
        }
    }

    // TODO: feel free to define other functions for your own use

}

// TODO: feel free to define other types (here or in other modules) for your own use
