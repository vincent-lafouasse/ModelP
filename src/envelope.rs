use crate::midi;
use cpal::OutputStreamTimestamp;
use std::time::Duration;

pub struct Envelope {
    attack: Duration,
    release: Duration,
}

type StreamEventKind = crate::midi::MidiEventKind;
pub struct StreamEvent {
    kind: StreamEventKind,
    timestamp: OutputStreamTimestamp,
}

impl Envelope {
    pub fn new(attack: Duration, release: Duration) -> Self {
        Self { attack, release }
    }

    pub fn volume(&self, now: OutputStreamTimestamp, last_event: Option<StreamEvent>) -> f32 {
        if let Some(StreamEvent { kind, timestamp }) = last_event {
            let full_duration = match kind {
                StreamEventKind::NoteOn => self.attack,
                StreamEventKind::NoteOff => self.release,
            };
            let advancement: Duration = now.playback.duration_since(&timestamp.playback).unwrap();
            let advancement = advancement.as_nanos() as f32 / full_duration.as_nanos() as f32;
            let advancement = advancement.clamp(0.0, 1.0);

            match kind {
                StreamEventKind::NoteOn => advancement,
                StreamEventKind::NoteOff => 1.0 - advancement,
            }
        } else {
            0.0
        }
    }
}
