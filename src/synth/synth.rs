use std::f32::consts::PI;
use std::sync::{mpsc, Arc};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;

use crate::event::Event;
use crate::midi::MidiNote;
use crate::synth::tuner::Tuner;
use crate::synth::wavetable::{Wavetable, WavetableBank, WavetableKind};

#[derive(Copy, Clone, Debug)]
pub struct Envelope {
    pub attack_ms: u16,
    pub decay_ms: u16,
    pub sustain: f32,
    pub release_ms: u16,
}

impl Envelope {
    fn new(attack_ms: u16, decay_ms: u16, sustain: f32, release_ms: u16) -> Self {
        Self {
            attack_ms,
            decay_ms,
            sustain,
            release_ms,
        }
    }

    pub fn default() -> Self {
        Envelope::new(5, 100, 0.7, 150)
    }

    fn attack_increment(&self, sample_rate: f32) -> f32 {
        1000.0 / (sample_rate * self.attack_ms as f32)
    }

    fn decay_increment(&self, sample_rate: f32) -> f32 {
        1000.0 * (1.0 - self.sustain) / (sample_rate * self.decay_ms as f32)
    }

    fn release_decrement(&self, sample_rate: f32) -> f32 {
        1000.0 / (sample_rate * self.release_ms as f32)
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum VoiceState {
    Idle,
    Attacking(MidiNote),
    Decaying(MidiNote),
    Sustaining(MidiNote),
    Releasing(MidiNote),
}

impl VoiceState {
    fn get_note(&self) -> Option<MidiNote> {
        match self {
            VoiceState::Idle => None,
            VoiceState::Attacking(note) => Some(*note),
            VoiceState::Decaying(note) => Some(*note),
            VoiceState::Sustaining(note) => Some(*note),
            VoiceState::Releasing(note) => Some(*note),
        }
    }
}

struct AudioThreadState {
    voice_state: VoiceState,
    wavetable_bank: Arc<WavetableBank>,
    wavetable_kind: WavetableKind,
    message_rx: mpsc::Receiver<Event>,
    volume: f32,
    master: f32,
    phase: f32,
    update_period: usize,
    update_timer: usize,
}

impl AudioThreadState {
    fn set_state(&mut self, voice_state: VoiceState) {
        dbg!(&voice_state);
        self.voice_state = voice_state;
    }
}

pub struct Synth {
    message_tx: mpsc::Sender<Event>,
    _stream: Stream,
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
        let mut envelope = Envelope::default();
        let mut tuner = Tuner::default();
        let mut state = AudioThreadState {
            voice_state: VoiceState::Idle,
            wavetable_bank: Arc::new(WavetableBank::new()),
            wavetable_kind: WavetableKind::Triangle,
            message_rx,
            volume: 0.0,
            master: 0.7,
            phase: 0.0,
            update_period: 5,
            update_timer: 0,
        };
        dbg!(&envelope);

        let callback = move |data: &mut [f32], _info: &cpal::OutputCallbackInfo| {
            'message_loop: loop {
                let event = state.message_rx.try_recv();
                if event.is_err() {
                    break 'message_loop;
                }
                dbg!(&event.unwrap());

                match event.unwrap() {
                    Event::NoteOn(incoming_note) => {
                        state.set_state(VoiceState::Attacking(incoming_note));
                    }
                    Event::NoteOff(incoming_note) => {
                        let current_note = state.voice_state.get_note();
                        if current_note.is_some() && current_note.unwrap() != incoming_note {
                            continue 'message_loop;
                        }
                        state.set_state(VoiceState::Releasing(incoming_note));
                    }
                    Event::OctaveUp => tuner.octave_up(),
                    Event::OctaveDown => tuner.octave_down(),
                    Event::ChangeOscillator(osc) => state.wavetable_kind = osc,
                    Event::SetMaster(master) => state.master = master,
                    Event::SetAttackMs(ms) => envelope.attack_ms = ms,
                    Event::SetDecayMs(ms) => envelope.decay_ms = ms,
                    Event::SetSustain(sustain) => envelope.sustain = sustain,
                    Event::SetReleaseMs(ms) => envelope.release_ms = ms,
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
                let new_sample = state.master
                    * state.volume
                    * state
                        .wavetable_bank
                        .get(state.wavetable_kind)
                        .at(state.phase);
                *sample = new_sample;
                state.phase += 2.0 * PI * frequency / sample_rate;
                state.phase = state.phase.rem_euclid(2.0 * PI);

                if state.update_timer % state.update_period == 0 {
                    if let VoiceState::Attacking(note) = state.voice_state {
                        if state.volume >= 1.0 {
                            state.volume = 1.0;
                            state.set_state(VoiceState::Decaying(note));
                        } else {
                            state.volume +=
                                state.update_period as f32 * envelope.attack_increment(sample_rate);
                            state.volume = f32::min(state.volume, 1.0);
                        }
                    } else if let VoiceState::Decaying(note) = state.voice_state {
                        if state.volume <= envelope.sustain {
                            state.volume = envelope.sustain;
                            state.set_state(VoiceState::Sustaining(note));
                        } else {
                            state.volume -=
                                state.update_period as f32 * envelope.decay_increment(sample_rate);
                            state.volume = f32::max(state.volume, envelope.sustain);
                        }
                    } else if let VoiceState::Releasing(_) = state.voice_state {
                        if state.volume <= 0.0 {
                            state.volume = 0.0;
                            state.set_state(VoiceState::Idle);
                        } else {
                            state.volume -= state.update_period as f32
                                * envelope.release_decrement(sample_rate);
                            state.volume = f32::max(state.volume, 0.0);
                        }
                    }
                    if state.update_timer == 20 * state.update_period {
                        state.update_timer = 0;
                        //dbg!(state.volume);
                    } else {
                        state.update_timer += 1;
                    }
                } else {
                    state.update_timer += 1;
                }
            }
        };

        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
        let _stream = device
            .build_output_stream(&stream_config.config(), callback, err_fn, None)
            .expect("failed to open output stream");
        let _ = _stream.play();

        Self {
            message_tx,
            _stream,
        }
    }

    pub fn send_event(&mut self, event: Event) {
        let _ = self.message_tx.send(event);
    }
}
