use std::f32::consts::PI;
use std::f32::consts::TAU;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::{mpsc, Arc};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;

use crate::envelope::StreamEvent;
use crate::midi::{MidiEvent, MidiEventKind, MidiNote};
use crate::wavetable::{Wavetable, WavetableBank, WavetableKind};

#[derive(PartialEq)]
enum VoiceState {
    Idle,
    Attacking(MidiNote),
    Sustaining(MidiNote),
    Releasing(MidiNote),
}

impl VoiceState {
    fn get_note(&self) -> Option<MidiNote> {
        match self {
            VoiceState::Idle => None,
            VoiceState::Attacking(note) => Some(*note),
            VoiceState::Sustaining(note) => Some(*note),
            VoiceState::Releasing(note) => Some(*note),
        }
    }
}

struct AudioThreadState {
    voice_state: VoiceState,
    wavetable: Arc<Wavetable>,
    message_rx: mpsc::Receiver<MidiEvent>,
    volume: f32,
    phase: f32,
}

impl AudioThreadState {
    fn new(wavetable: Arc<Wavetable>, message_rx: mpsc::Receiver<MidiEvent>) -> Self {
        Self {
            voice_state: VoiceState::Idle,
            wavetable,
            message_rx,
            volume: 0.0,
            phase: 0.0,
        }
    }
}

pub struct Synth {
    message_tx: mpsc::Sender<MidiEvent>,
    stream: Stream,
}

impl Synth {
    pub fn new() -> Self {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("no output device available");

        let mut supported_configs_range = device
            .supported_output_configs()
            .expect("error while querying configs");
        let stream_config = supported_configs_range
            .next()
            .expect("no supported config?!")
            .with_max_sample_rate();
        let sample_rate = stream_config.sample_rate().0;

        let (message_tx, message_rx) = mpsc::channel::<MidiEvent>();

        // vvv moved into thread
        let wavetable_bank: Arc<WavetableBank> = Arc::new(WavetableBank::new());
        let mut state =
            AudioThreadState::new(wavetable_bank.get(WavetableKind::TriangleSaw), message_rx);

        let callback = move |data: &mut [f32], info: &cpal::OutputCallbackInfo| {
            'message_loop: loop {
                match state.message_rx.try_recv() {
                    Err(_) => {
                        break 'message_loop;
                    }
                    Ok(event) => match event.kind {
                        MidiEventKind::NoteOn => {
                            state.voice_state = VoiceState::Attacking(event.note);
                        }
                        MidiEventKind::NoteOff => {
                            state.voice_state = VoiceState::Idle;
                            state.phase = 0.0;
                        }
                    },
                }
            }
            if state.voice_state == VoiceState::Idle {
                for sample in data {
                    *sample = cpal::Sample::EQUILIBRIUM;
                }
                return;
            }
            let frequency: f32 = state.voice_state.get_note().unwrap().frequency();
            let volume: f32 = match state.voice_state {
                VoiceState::Attacking(..)
                | VoiceState::Sustaining(..)
                | VoiceState::Releasing(..) => 1.0,
                VoiceState::Idle => unreachable!(),
            };
            for sample in data {
                *sample = volume * state.wavetable.at(state.phase);
                state.phase += 2.0 * PI * frequency / sample_rate as f32;
                state.phase = state.phase.rem_euclid(2.0 * PI);
            }
        };

        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
        let stream = device
            .build_output_stream(&stream_config.config(), callback, err_fn, None)
            .expect("failed to open output stream");
        let _ = stream.play();

        Self { message_tx, stream }
    }

    pub fn send_midi_event(&mut self, event: MidiEvent) {
        let _ = self.message_tx.send(event);
    }
}
