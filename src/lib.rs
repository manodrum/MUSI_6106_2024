mod ring_buffer;
mod comb_filter;
mod dancing_strings;

use comb_filter::{CombFilter, FilterParam, FilterType};
use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui, widgets, EguiState};
use std::sync::Arc;

// This is a shortened version of the gain example with most comments removed, check out
// https://github.com/robbert-vdh/nih-plug/blob/master/plugins/examples/gain/src/lib.rs to get
// started

struct AseExample {
    params: Arc<AseExampleParams>,
    comb_filter: Option<CombFilter>,
}

#[derive(Params)]
struct AseExampleParams {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    editor_state: Arc<EguiState>,
    /// The parameter's ID is used to identify the parameter in the wrappred plugin API. As long as
    /// these IDs remain constant, you can rename and reorder these fields as you wish. The
    /// parameters are exposed to the host in the same order they were defined. In this case, this
    /// gain parameter is stored as linear gain while the values are displayed in decibels.
    #[id = "gain"]
    pub gain: FloatParam,
    #[id = "delay"]
    pub delay: FloatParam,
    #[id = "filter_type"]
    pub filter_type: EnumParam<FilterType>,
}

impl Default for AseExample {
    fn default() -> Self {
        Self {
            params: Arc::new(AseExampleParams::default()),
            comb_filter: None,
        }
    }
}

impl Default for AseExampleParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(400, 340),
            // This gain is stored as linear gain. NIH-plug comes with useful conversion functions
            // to treat these kinds of parameters as if we were dealing with decibels. Storing this
            // as decibels is easier to work with, but requires a conversion for every sample.
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    // This makes the range appear as if it was linear when displaying the values as
                    // decibels
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            // Because the gain parameter is stored as linear gain instead of storing the value as
            // decibels, we need logarithmic smoothing
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit("dB")
            // There are many predefined formatters we can use here. If the gain was stored as
            // decibels instead of as a linear gain value, we could have also used the
            // `.with_step_size(0.1)` function to get internal rounding.
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            delay: FloatParam::new(
                "Delay",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1000.0 },
            )
            .with_unit("ms")
            .with_step_size(1.0),

            filter_type: EnumParam::new("Filter Type", FilterType::FIR),
        }
    }
}

impl Plugin for AseExample {
    const NAME: &'static str = "Ase Example";
    const VENDOR: &'static str = "Ian Clester";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "ijc@gatech.edu";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        // Individual ports and the layout as a whole can be named here. By default these names
        // are generated as needed. This layout will be called 'Stereo', while a layout with
        // only one input and output channel would be called 'Mono'.
        names: PortNames::const_default(),
    }];


    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let params = self.params.clone();
        create_egui_editor(
            self.params.editor_state.clone(),
            (),
            |_, _| {},
            move |egui_ctx, setter, _state| {
                egui::CentralPanel::default().show(egui_ctx, |ui| {
                    // NOTE: See `plugins/diopser/src/editor.rs` for an example using the generic UI widget

                    // This is a fancy widget that can get all the information it needs to properly
                    // display and modify the parameter from the parametr itself
                    // It's not yet fully implemented, as the text is missing.
                    ui.label("Gain");
                    ui.add(widgets::ParamSlider::for_param(&params.gain, setter));

                    ui.label("Delay");
                    ui.add(widgets::ParamSlider::for_param(&params.delay, setter));

                    ui.label(
                        "Also gain, but with a lame widget. Can't even render the value correctly!",
                    );
                    // This is a simple naieve version of a parameter slider that's not aware of how
                    // the parameters work
                    ui.add(
                        egui::widgets::Slider::from_get_set(-30.0..=30.0, |new_value| {
                            match new_value {
                                Some(new_value_db) => {
                                    let new_value = util::gain_to_db(new_value_db as f32);

                                    setter.begin_set_parameter(&params.gain);
                                    setter.set_parameter(&params.gain, new_value);
                                    setter.end_set_parameter(&params.gain);

                                    new_value_db
                                }
                                None => util::gain_to_db(params.gain.value()) as f64,
                            }
                        })
                        .suffix(" dB"),
                    );

                    ui.label("Cool custom widget");
                    dancing_strings::draw(ui);
                });
            },
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        let mut comb_filter = CombFilter::new(
            FilterType::FIR,
            1.0,
            _buffer_config.sample_rate,
            _audio_io_layout.main_input_channels.unwrap().get() as usize
        );
        comb_filter.set_param(FilterParam::Delay, 1.0).unwrap();
        comb_filter.set_param(FilterParam::Gain, 0.8).unwrap();
        dbg!(comb_filter.get_param(FilterParam::Delay));
        self.comb_filter = Some(comb_filter);
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
        self.comb_filter.as_mut().unwrap().reset()
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let gain = self.params.gain.value();
        let delay = self.params.delay.value() / 1000.0;
        let comb_filter = self.comb_filter.as_mut().unwrap();
        comb_filter.filter_type = self.params.filter_type.value();
        comb_filter.set_param(FilterParam::Gain, gain).unwrap();
        comb_filter.set_param(FilterParam::Delay, delay).unwrap();
        comb_filter.process(buffer.as_slice());

        ProcessStatus::Normal
    }
}

impl ClapPlugin for AseExample {
    const CLAP_ID: &'static str = "edu.gatech.ase-example";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Simple example plugin for Audio Software Engineering.");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for AseExample {
    const VST3_CLASS_ID: [u8; 16] = *b"ASE-2024-Example";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_clap!(AseExample);
nih_export_vst3!(AseExample);
