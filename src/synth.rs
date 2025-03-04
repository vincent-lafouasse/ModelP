use std::f32::consts::PI;
use std::f32::consts::TAU;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;

use crate::midi::{MidiEvent, MidiEventKind, MidiNote};
use crate::wavetable::Wavetable;

struct AudioThreadState {
    frequency_bits: Arc<AtomicU32>,
    playing: Arc<AtomicBool>,
    volume: f32,
    phase: f32,
}

impl AudioThreadState {
    fn new(frequency_bits: Arc<AtomicU32>, playing: Arc<AtomicBool>) -> Self {
        Self {
            frequency_bits,
            playing,
            volume: 0.0,
            phase: 0.0,
        }
    }
}

pub struct Synth {
    frequency_bits: Arc<AtomicU32>,
    playing: Arc<AtomicBool>,
    current_note: Option<MidiNote>,
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

        let frequency_bits: Arc<AtomicU32> = Arc::new(AtomicU32::new(0));
        frequency_bits.store(Into::<f32>::into(256.0f32).to_bits(), Ordering::Relaxed);
        let playing: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

        // vvv moved into thread
        let mut state = AudioThreadState::new(frequency_bits.clone(), playing.clone());
        let wavetable: Wavetable = Wavetable::sine();
        let callback = move |data: &mut [f32], info: &cpal::OutputCallbackInfo| {
            let frequency: f32 = f32::from_bits(state.frequency_bits.load(Ordering::Relaxed));
            let volume: f32 = match state.playing.load(Ordering::Relaxed) {
                true => 1.0,
                false => 0.0,
            };
            for sample in data {
                *sample = volume * wavetable.at(state.phase);
                state.phase += 2.0 * PI * frequency / sample_rate as f32;
                state.phase = state.phase.rem_euclid(2.0 * PI);
            }
        };
        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
        let stream = device
            .build_output_stream(&stream_config.config(), callback, err_fn, None)
            .expect("failed to open output stream");
        let _ = stream.play();

        Self {
            frequency_bits,
            playing,
            current_note: None,
            stream,
        }
    }

    pub fn set_frequency(&mut self, f: f32) {
        dbg!(f);
        self.frequency_bits
            .store(Into::<f32>::into(f).to_bits(), Ordering::Relaxed);
    }

    pub fn send_midi_event(&mut self, event: MidiEvent) {}

    fn process_note_on(&mut self, note: MidiNote) {
        self.playing.store(true, Ordering::Relaxed);
        self.current_note = Some(note);
        self.set_frequency(note.frequency());
    }

    fn process_note_off(&mut self, note: MidiNote) {
        if self.current_note.is_none() {
            return;
        }

        let current_note = self.current_note.unwrap();
        if current_note != note {
            return;
        }

        self.playing.store(false, Ordering::Relaxed);
        self.current_note = None;
    }
}
