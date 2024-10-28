#![allow(unused)]

use std::f32::consts::PI;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Data, Stream};

const WAVETABLE_RESOLUTION: usize = 256;

fn build_sine_wavetable(resolution: usize) -> Arc<[f32]> {
    (0..resolution)
        .map(|i| 2.0 * PI * (i as f32) / (resolution as f32))
        .map(|phase| phase.sin())
        .collect()
}

struct Synth {
    wavetable: Arc<[f32]>,
    frequency_bits: Arc<AtomicU32>,
    stream: Stream,
}

impl Synth {
    fn new() -> Self {
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
        let sample_rate = stream_config.sample_rate().0;

        let wavetable = build_sine_wavetable(WAVETABLE_RESOLUTION);
        let frequency_bits: Arc<AtomicU32> = Arc::new(AtomicU32::new(0));
        frequency_bits.store(Into::<f32>::into(256.0f32).to_bits(), Ordering::Relaxed);

        let mut phase: f32 = 0.0;
        let frequency_bits__ = frequency_bits.clone();
        let callback = move |data: &mut [f32], info: &cpal::OutputCallbackInfo| {
            let frequency: f32 = f32::from_bits(frequency_bits__.load(Ordering::Relaxed));
            for sample in data {
                *sample = phase.sin();
                phase = phase + 2.0 * PI * frequency / sample_rate as f32;
                phase = phase.rem_euclid(2.0 * PI);
            }
        };
        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
        let stream = device
            .build_output_stream(&stream_config.config(), callback, err_fn, None)
            .expect("failed to open output stream");
        stream.play();

        Self {
            wavetable,
            frequency_bits,
            stream,
        }
    }
}

fn main() {
    let synth = Synth::new();
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
