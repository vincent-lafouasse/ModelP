#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MidiNote {
    pub note: u8,
}

impl MidiNote {
    pub fn new(note: u8) -> Self {
        Self { note }
    }

    // 12TET
    pub fn frequency(&self) -> f32 {
        let offset_from_a4: i16 = self.note as i16 - 69;

        440.0 * 2.0_f32.powf(offset_from_a4 as f32 / 12.0)
    }

    #[allow(dead_code)]
    pub fn offset_up(&self, n: u8) -> Self {
        let note: u8 = self.note.saturating_add(n);
        let note: u8 = if note > 127 { 127 } else { note };
        Self { note }
    }

    #[allow(dead_code)]
    pub fn offset_down(&self, n: u8) -> Self {
        let note: u8 = self.note.saturating_sub(n);
        Self { note }
    }

    #[allow(dead_code)]
    pub fn octave_up(&self, n: u8) -> Self {
        self.offset_up(12 * n)
    }

    #[allow(dead_code)]
    pub fn octave_down(&self, n: u8) -> Self {
        self.offset_down(12 * n)
    }

    #[allow(dead_code)]
    pub fn c0() -> Self {
        Self { note: 12 }
    }

    #[allow(dead_code)]
    pub fn c(octave: u8) -> Self {
        Self::c0().octave_up(octave)
    }
}
