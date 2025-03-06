use std::f32::consts::TAU;
use std::sync::Arc;

use hound::{SampleFormat, WavReader, WavSpec};

const WAVETABLE_RESOLUTION: usize = 256;
const FLAT_GAIN_REDUCTION: f32 = 0.7;

pub const TRIANGLE_WAVETABLE_PATH: &'static str = "./assets/wavetables/mini_triangle_wavetable.wav";

#[derive(Clone, Debug)]
pub struct Wavetable {
    data: Arc<[f32]>,
    size: usize,
}

impl Wavetable {
    pub fn from_disk(path: &str) -> Self {
        let reader = WavReader::open(path).unwrap();
        let size: usize = reader.len() as usize;
        let data: Vec<f32> = match reader.spec() {
            WavSpec {
                sample_format: SampleFormat::Int,
                ..
            } => reader
                .into_samples::<i32>()
                .map(|x| x.unwrap())
                .map(|x| if x == i32::MIN { i32::MIN + 1 } else { x })
                .map(|x| x as f32 / i32::MAX as f32)
                .collect(),
            WavSpec {
                sample_format: SampleFormat::Float,
                ..
            } => reader.into_samples::<f32>().map(|x| x.unwrap()).collect(),
        };
        let infinite_norm: f32 = data
            .iter()
            .map(|x: &f32| x.abs())
            .fold(0.0, |max, x| f32::max(max, x));

        let data: Vec<f32> = if infinite_norm > 0.0 {
            data.iter()
                .map(|x| FLAT_GAIN_REDUCTION * *x / infinite_norm)
                .collect()
        } else {
            data
        };
        let data: Arc<[f32]> = Arc::from(data);

        Self { data, size }
    }

    pub fn sine() -> Self {
        let size = WAVETABLE_RESOLUTION;
        let data: Arc<[f32]> = (0..size)
            .map(|i| TAU * (i as f32) / (size as f32))
            .map(|phase| phase.sin())
            .collect();

        Self { data, size }
    }

    // 2π periodic
    pub fn at(&self, phase: f32) -> f32 {
        let float_index = self.size as f32 * phase.rem_euclid(TAU) / TAU;
        let lower: usize = float_index.floor() as usize;
        let higher: usize = wrapped_increment(lower, self.size - 1);

        crate::math::lerp(float_index.fract(), self.data[lower], self.data[higher])
    }
}

fn wrapped_increment(n: usize, max: usize) -> usize {
    if n == max {
        0
    } else {
        n + 1
    }
}
