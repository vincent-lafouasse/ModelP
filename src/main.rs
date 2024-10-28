#![allow(unused)]

use std::f32::consts::PI;
use std::sync::Arc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Data;

const WAVETABLE_RESOLUTION: usize = 256;

fn main() {
    let sine_wavetable: Vec<f32> = (0..WAVETABLE_RESOLUTION)
        .map(|i| 2.0 * PI * (i as f32) / (WAVETABLE_RESOLUTION as f32))
        .map(|phase| phase.sin())
        .collect();
    let sine_wavetable: Arc<[f32]> = sine_wavetable.into();

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
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // react to stream events and read or write stream data here.
            },
            move |err| {
                // react to errors here.
            },
            None, // None=blocking, Some(Duration)=timeout
        )
        .expect("failed to open output stream");

    stream.play();
    std::thread::sleep(std::time::Duration::from_secs(1));
}
