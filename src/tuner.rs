use crate::midi::MidiNote;

pub struct Tuner {
    a4: f32,
}

impl Default for Tuner {
    fn default() -> Self {
        Self {a4: 440.0}
    }
}

impl Tuner {
    fn get(&self, note: MidiNote) -> f32 {
        let offset_from_a4: i16 = note.note as i16 - 69;

        440.0 * 2.0_f32.powf(offset_from_a4 as f32 / 12.0)
    }

    fn octave_up(&mut self) {
        self.a4 *= 2.0;
    }

    fn octave_down(&mut self) {
        self.a4 /= 2.0;
    }
}