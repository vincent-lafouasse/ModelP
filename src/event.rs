use crate::midi::MidiNote;

#[derive(PartialEq, Debug)]
pub enum Event {
    NoteOn(MidiNote),
    NoteOff(MidiNote),
    OctaveUp,
    OctaveDown,
}

impl Event {}
