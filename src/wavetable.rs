use std::f32::consts::TAU;
use std::sync::Arc;

use hound::{SampleFormat, WavReader, WavSpec};

#[allow(dead_code)]
const WAVETABLE_RESOLUTION: usize = 256;
const FLAT_GAIN_REDUCTION: f32 = 0.7;

pub struct WavetableBank {
    triangle: Arc<Wavetable>,
    triangle_saw: Arc<Wavetable>,
    saw: Arc<Wavetable>,
    square: Arc<Wavetable>,
    pwm_wide: Arc<Wavetable>,
    pwm_narrow: Arc<Wavetable>,
}

impl WavetableBank {
    pub fn new() -> Self {
        let triangle: Arc<Wavetable> =
            Arc::new(Wavetable::from_disk(WavetableKind::Triangle.path()));
        let triangle_saw: Arc<Wavetable> =
            Arc::new(Wavetable::from_disk(WavetableKind::TriangleSaw.path()));
        let saw: Arc<Wavetable> = Arc::new(Wavetable::from_disk(WavetableKind::Saw.path()));
        let square: Arc<Wavetable> = Arc::new(Wavetable::from_disk(WavetableKind::Square.path()));
        let pwm_wide: Arc<Wavetable> =
            Arc::new(Wavetable::from_disk(WavetableKind::PulseWide.path()));
        let pwm_narrow: Arc<Wavetable> =
            Arc::new(Wavetable::from_disk(WavetableKind::PulseNarrow.path()));

        Self {
            triangle,
            triangle_saw,
            saw,
            square,
            pwm_wide,
            pwm_narrow,
        }
    }

    pub fn get(&self, kind: WavetableKind) -> Arc<Wavetable> {
        match kind {
            WavetableKind::Triangle => self.triangle.clone(),
            WavetableKind::TriangleSaw => self.triangle_saw.clone(),
            WavetableKind::Saw => self.saw.clone(),
            WavetableKind::Square => self.square.clone(),
            WavetableKind::PulseWide => self.pwm_wide.clone(),
            WavetableKind::PulseNarrow => self.pwm_narrow.clone(),
        }
    }
}

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
        let infinite_norm: f32 = data.iter().map(|x: &f32| x.abs()).fold(0.0, f32::max);

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

    #[allow(dead_code)]
    pub fn pure_sine() -> Self {
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

#[derive(Copy, Clone)]
pub enum WavetableKind {
    Triangle,
    TriangleSaw,
    Saw,
    Square,
    PulseWide,
    PulseNarrow,
}

impl WavetableKind {
    pub fn path(&self) -> &'static str {
        match self {
            WavetableKind::Triangle => "./assets/wavetables/mini_triangle_wavetable.wav",
            WavetableKind::TriangleSaw => "./assets/wavetables/mini_triangle_saw_wavetable.wav",
            WavetableKind::Saw => "./assets/wavetables/mini_saw_wavetable.wav",
            WavetableKind::Square => "./assets/wavetables/mini_square_wavetable.wav",
            WavetableKind::PulseWide => "./assets/wavetables/mini_pwm_wide_wavetable.wav",
            WavetableKind::PulseNarrow => "./assets/wavetables/mini_pwm_narrow_wavetable.wav",
        }
    }
}

impl std::fmt::Display for WavetableKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr: &'static str = match self {
            WavetableKind::Triangle => "Triangle",
            WavetableKind::TriangleSaw => "TriangleSaw",
            WavetableKind::Saw => "Saw",
            WavetableKind::Square => "Square",
            WavetableKind::PulseWide => "PWM Wide",
            WavetableKind::PulseNarrow => "PWM Narrow",
        };
        write!(f, "{}", repr)
    }
}

fn wrapped_increment(n: usize, max: usize) -> usize {
    if n == max {
        0
    } else {
        n + 1
    }
}
