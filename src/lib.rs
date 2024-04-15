mod ring_buffer;
mod comb_filter;

use std::sync::mpsc;

use comb_filter::{CombFilter, FilterParam, FilterType};
use wasm_bindgen::prelude::*;
use web_sys::console;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SampleRate, SizedSample, Stream};

struct Message {
    param: FilterParam,
    value: f32,
}

#[wasm_bindgen]
pub struct Handle {
    stream: Stream,
    tx: mpsc::Sender<Message>,
}

#[wasm_bindgen]
impl Handle {
    pub fn set_delay(&mut self, delay: f32) {
        self.tx.send(Message { param: FilterParam::Delay, value: delay }).unwrap();
    }
    pub fn set_gain(&mut self, gain: f32) {
        self.tx.send(Message { param: FilterParam::Gain, value: gain }).unwrap();
    }
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Handle
where
    T: SizedSample + FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // TODO: Setup and initialize your comb filter.
    let (tx, rx) = mpsc::channel::<Message>();
    let mut filter = CombFilter::new(FilterType::IIR, 1.0, sample_rate, 1);
    filter.set_param(FilterParam::Delay, 0.2).unwrap();
    filter.set_param(FilterParam::Gain, 0.5).unwrap();
    let mut t = 0f32;

    let mut blips = move || {
        // Generate blips.
        t += 1.0 / sample_rate;
        if t >= 1.0 {
            t -= 1.0;
        }
        let dur = 1.0 / 8.0;
        let sqr = if t * 1000.0 % 1.0 < 0.5 { 1.0 } else { -1.0 };
        let env = if t > dur { 0.0 } else { (1.0 - t / dur).powf(4.0) };
        sqr * env
    };

    let mut next_value = {
        move || {
            while let Ok(m) = rx.try_recv() {
                filter.set_param(m.param, m.value).unwrap();
            }
            let sample = blips();
            let inp: &[&[f32]] = &[&[sample]];
            let out: &mut [&mut [f32]] = &mut [&mut [sample]];
            filter.process(inp, out);
            out[0][0]
        }
    };

    let err_fn = |err| console::error_1(&format!("an error occurred on stream: {}", err).into());

    let stream = device
        .build_output_stream(
            config,
            move |data: &mut [T], _| write_data(data, channels, &mut next_value),
            err_fn,
            None,
        )
        .unwrap();
    stream.play().unwrap();
    Handle { stream, tx }
}


// === cpal & wasm-bindgen boilerplate ===

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
pub fn play() -> Handle {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    // let mut config = device.default_output_config().unwrap();
    let configs = device.supported_output_configs().unwrap();
    let config = configs
        .filter(|c| c.channels() == 1)
        .max_by(|a, b| a.cmp_default_heuristics(b))
        .unwrap()
        .with_sample_rate(SampleRate(44100));

    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
        // not all supported sample formats are included in this example
        _ => panic!("Unsupported sample format!"),
    }
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: SizedSample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value: T = T::from_sample(next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
