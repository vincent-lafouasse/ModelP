use std::time::Instant;

pub struct MidiNote {
    offset: u8
}

impl MidiNote {
    pub fn new(offset: u8) -> Self {
        Self {offset}
    }

    // 12TET
    pub fn frequency(&self) -> f32 {
        let offset_from_a4: u8 = self.offset - 60;

        440.0 * 2.0_f32.powf(offset_from_a4 as f32 / 12.0)
    }
}

pub enum MidiEventKind {
    NoteOn,
    NoteOff,
}

pub struct MidiEvent {
    pub note: MidiNote,
    pub kind: MidiEventKind,
    pub timestamp: Instant,
}

impl MidiEvent {
    pub fn note_on(note: u8) -> Self {
        let note = MidiNote::new(note);
        let kind = MidiEventKind::NoteOn;
        let timestamp = Instant::now();
        Self {
            note,
            kind,
            timestamp,
        }
    }

    pub fn note_off(note: u8) -> Self {
        let note = MidiNote::new(note);
        let kind = MidiEventKind::NoteOff;
        let timestamp = Instant::now();
        Self {
            note,
            kind,
            timestamp,
        }
    }
}
