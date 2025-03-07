use crate::midi::MidiNote;
use crate::wavetable::WavetableKind;

#[derive(PartialEq, Debug)]
pub enum Event {
    NoteOn(MidiNote),
    NoteOff(MidiNote),
    OctaveUp,
    OctaveDown,
    ChangeOscillator(WavetableKind),
    SetMaster(f32),
}

impl Event {}
