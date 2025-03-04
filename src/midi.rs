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
}
