use sdl2::sys::wchar_t;
use std::time::Instant;

pub struct MidiNote {
    offset: u8,
}

impl MidiNote {
    pub fn new(offset: u8) -> Self {
        Self { offset }
    }

    // 12TET
    pub fn frequency(&self) -> f32 {
        let offset_from_a4: u8 = self.offset - 69;

        440.0 * 2.0_f32.powf(offset_from_a4 as f32 / 12.0)
    }

    #[allow(dead_code)]
    pub fn c0() -> Self {
        let offset = 12;
        Self { offset }
    }

    #[allow(dead_code)]
    pub fn c1() -> Self {
        let offset = Self::c0().offset + 12;
        Self { offset }
    }

    #[allow(dead_code)]
    pub fn c2() -> Self {
        let offset = Self::c0().offset + 2 * 12;
        Self { offset }
    }

    #[allow(dead_code)]
    pub fn c3() -> Self {
        let offset = Self::c0().offset + 3 * 12;
        Self { offset }
    }

    #[allow(dead_code)]
    pub fn c4() -> Self {
        let offset = Self::c0().offset + 4 * 12;
        Self { offset }
    }

    #[allow(dead_code)]
    pub fn c5() -> Self {
        let offset = Self::c0().offset + 5 * 12;
        Self { offset }
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
