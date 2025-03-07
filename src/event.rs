use crate::midi::MidiNote;

#[derive(PartialEq)]
pub enum EventKind {
    NoteOn,
    NoteOff,
}

pub struct Event {
    pub note: MidiNote,
    pub kind: EventKind,
}

impl Event {
    pub fn new(note: MidiNote, kind: EventKind) -> Self {
        Self { note, kind }
    }

    pub fn note_on(note: MidiNote) -> Self {
        let kind = EventKind::NoteOn;
        Self { note, kind }
    }

    pub fn note_off(note: MidiNote) -> Self {
        let kind = EventKind::NoteOff;
        Self { note, kind }
    }
}
