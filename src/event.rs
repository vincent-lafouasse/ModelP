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
    SetAttackMs(u16),
    SetDecayMs(u16),
    SetSustain(f32),
    SetReleaseMs(u16),
}

impl Event {}
