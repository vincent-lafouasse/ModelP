use std::time::Instant;

pub enum MidiEventKind {
    NoteOn,
    NoteOff,
}

pub struct MidiEvent {
    pub note: u8,
    pub kind: MidiEventKind,
    pub timestamp: Instant,
}

impl MidiEvent {
    pub fn note_on(note: u8) -> Self {
        let kind = MidiEventKind::NoteOn;
        let timestamp = Instant::now();
        Self {
            note,
            kind,
            timestamp,
        }
    }

    pub fn note_off(note: u8) -> Self {
        let kind = MidiEventKind::NoteOff;
        let timestamp = Instant::now();
        Self {
            note,
            kind,
            timestamp,
        }
    }

    // 12TET
    pub fn frequency(&self) -> f32 {
        let offset_from_a4: u8 = self.note - 60;

        440.0 * 2.0_f32.powf(offset_from_a4 as f32 / 12.0)
    }
}
