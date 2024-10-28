#![allow(unused)]

use std::f32::consts::PI;
use std::sync::Arc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Data;

const WAVETABLE_RESOLUTION: usize = 256;

fn build_sine_wavetable(resolution: usize) -> Arc<[f32]> {
    (0..resolution)
        .map(|i| 2.0 * PI * (i as f32) / (resolution as f32))
        .map(|phase| phase.sin())
        .collect()
}

struct Synth {
    wavetable: Arc<[f32]>,
    phase: f32,
}

impl Synth {
    fn new() -> Self {
        let wavetable = build_sine_wavetable(WAVETABLE_RESOLUTION);
        let phase = 0.0;

        Self { wavetable, phase }
    }

    fn open_stream(&self) {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");

        let mut supported_configs_range = device
            .supported_output_configs()
            .expect("error while querying configs");
        let stream_config = supported_configs_range
            .next()
            .expect("no supported config?!")
            .with_max_sample_rate();
        let stream = device
            .build_output_stream(
                &stream_config.config(),
                move |data: &mut [f32], info: &cpal::OutputCallbackInfo| {
                    callback(data, info);
                },
                move |err| {
                    // react to errors here.
                },
                None,
            )
            .expect("failed to open output stream");
        stream.play();
    }
}

fn callback(data: &mut [f32], info: &cpal::OutputCallbackInfo) {
    for sample in data {
        *sample = 0.0;
    }
}

fn main() {
    let synth = Synth::new();
    synth.open_stream();
}
