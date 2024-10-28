#![allow(unused)]

use std::f32::consts::PI;
use std::sync::Arc;

const WAVETABLE_RESOLUTION: usize = 256;

fn main() {
    let sine_wavetable: Vec<f32> = (0..WAVETABLE_RESOLUTION)
        .map(|i| 2.0 * PI * (i as f32) / (WAVETABLE_RESOLUTION as f32))
        .map(|phase| phase.sin())
        .collect();
    let sine_wavetable: Arc<[f32]> = sine_wavetable.into();

    dbg!(sine_wavetable);
}
