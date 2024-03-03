
pub struct Test {
    freq: f32,  // Frequency of LFO modulation
}

impl Test {

    pub fn new(mod_freq: f32) -> Self {
        Test {
            freq: 20.0,  // Will be set properly using set_param
        }
    }


    pub fn get_param(&self, param: &str) -> f32 {
        match param {
            "frequency" => self.freq,
            _ => unimplemented!(),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*; // Import the Vibrato struct and any other necessary items from the outer module


    #[test]
    fn run() {
        let new_test = Test::new(30.0);
        let result = new_test.get_param("frequency");
        println!("result: {:?}", result);
    }

}