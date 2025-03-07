use std::f32::consts::PI;
use std::sync::{mpsc, Arc};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;

use crate::event::Event;
use crate::midi::MidiNote;
use crate::wavetable::{Wavetable, WavetableBank, WavetableKind};

struct Envelope {
    attack_ms: u16,
    release_ms: u16,
}

impl Envelope {
    fn new(attack_ms: u16, release_ms: u16) -> Self {
        Self {
            attack_ms,
            release_ms,
        }
    }

    fn attack_increment(&self, sample_rate: f32) -> f32 {
        1000.0 / (sample_rate * self.attack_ms as f32)
    }

    fn release_decrement(&self, sample_rate: f32) -> f32 {
        1000.0 / (sample_rate * self.release_ms as f32)
    }
}

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
    message_rx: mpsc::Receiver<Event>,
    volume: f32,
    phase: f32,
    update_period: usize,
    update_timer: usize,
}

pub struct Synth {
    message_tx: mpsc::Sender<Event>,
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
        let sample_rate: f32 = stream_config.sample_rate().0 as f32;

        let (message_tx, message_rx) = mpsc::channel::<Event>();

        // vvv moved into thread
        let envelope = Envelope::new(1500, 3000);
        let wavetable_bank: Arc<WavetableBank> = Arc::new(WavetableBank::new());
        let mut tuner = crate::tuner::Tuner::default();
        let mut state = AudioThreadState {
            voice_state: VoiceState::Idle,
            wavetable: wavetable_bank.get(WavetableKind::Triangle),
            message_rx,
            volume: 0.0,
            phase: 0.0,
            update_period: 5,
            update_timer: 0,
        };

        let callback = move |data: &mut [f32], _info: &cpal::OutputCallbackInfo| {
            'message_loop: loop {
                let event = state.message_rx.try_recv();
                if event.is_err() {
                    break 'message_loop;
                }

                let event = event.unwrap();
                if let Event::NoteOn(incoming_note) = event {
                    state.voice_state = VoiceState::Attacking(incoming_note);
                } else if let Event::NoteOff(incoming_note) = event {
                    let current_note = state.voice_state.get_note();
                    if current_note.is_some() && current_note.unwrap() != incoming_note {
                        continue 'message_loop;
                    }
                    state.voice_state = VoiceState::Releasing(incoming_note);
                } else if let Event::OctaveUp = event {
                    tuner.octave_up();
                } else if let Event::OctaveDown = event {
                    tuner.octave_down();
                }
            }
            if state.voice_state == VoiceState::Idle {
                for sample in data {
                    *sample = cpal::Sample::EQUILIBRIUM;
                }
                state.volume = 0.0;
                return;
            }
            let frequency: f32 = tuner.get(state.voice_state.get_note().unwrap());
            for sample in data {
                let new_sample = state.volume * state.wavetable.at(state.phase);
                *sample = new_sample;
                state.phase += 2.0 * PI * frequency / sample_rate;
                state.phase = state.phase.rem_euclid(2.0 * PI);

                if state.update_timer == state.update_period - 1 {
                    if let VoiceState::Attacking(note) = state.voice_state {
                        if state.volume >= 1.0 {
                            state.volume = 1.0;
                            state.voice_state = VoiceState::Sustaining(note);
                        } else {
                            state.volume +=
                                state.update_period as f32 * envelope.attack_increment(sample_rate);
                            state.volume = f32::min(state.volume, 1.0);
                        }
                    } else if let VoiceState::Releasing(_) = state.voice_state {
                        if state.volume <= 0.0 {
                            state.volume = 0.0;
                            state.voice_state = VoiceState::Idle;
                        } else {
                            state.volume -= state.update_period as f32
                                * envelope.release_decrement(sample_rate);
                            state.volume = f32::max(state.volume, 0.0);
                        }
                    }
                    state.update_timer = 0;
                } else {
                    state.update_timer += 1;
                }
            }
        };

        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
        let stream = device
            .build_output_stream(&stream_config.config(), callback, err_fn, None)
            .expect("failed to open output stream");
        let _ = stream.play();

        Self { message_tx, stream }
    }

    pub fn send_midi_event(&mut self, event: Event) {
        let _ = self.message_tx.send(event);
    }
}
