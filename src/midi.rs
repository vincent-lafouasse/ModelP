use sdl2::sys::wchar_t;
use std::time::Instant;

pub struct MidiNote {
    note: u8,
}

impl MidiNote {
    pub fn new(note: u8) -> Self {
        Self { note }
    }

    // 12TET
    pub fn frequency(&self) -> f32 {
        let offset_from_a4: u8 = self.note - 69;

        440.0 * 2.0_f32.powf(offset_from_a4 as f32 / 12.0)
    }

    #[allow(dead_code)]
    pub fn c0() -> Self {
        let note = 12;
        Self { note }
    }

    #[allow(dead_code)]
    pub fn c1() -> Self {
        let note = Self::c0().note + 12;
        Self { note }
    }

    #[allow(dead_code)]
    pub fn c2() -> Self {
        let note = Self::c0().note + 2 * 12;
        Self { note }
    }

    #[allow(dead_code)]
    pub fn c3() -> Self {
        let note = Self::c0().note + 3 * 12;
        Self { note }
    }

    #[allow(dead_code)]
    pub fn c4() -> Self {
        let note = Self::c0().note + 4 * 12;
        Self { note }
    }

    #[allow(dead_code)]
    pub fn c5() -> Self {
        let offset = Self::c0().note + 5 * 12;
        Self { note: offset }
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
