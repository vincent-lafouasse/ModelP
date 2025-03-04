use std::f32::consts::TAU;
use std::sync::Arc;

const WAVETABLE_RESOLUTION: usize = 256;

#[derive(Clone, Debug)]
pub struct Wavetable {
    data: Arc<[f32]>,
    size: usize,
}

impl Wavetable {
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
