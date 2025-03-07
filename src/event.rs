use crate::midi::MidiNote;

#[derive(PartialEq)]
pub enum Event {
    NoteOn(MidiNote),
    NoteOff(MidiNote),
    OctaveUp,
    OctaveDown,
}

impl Event {}
