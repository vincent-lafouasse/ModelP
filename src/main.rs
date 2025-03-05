#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::collections::HashSet;
use std::time::{Duration, Instant};

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

mod envelope;
mod math;
mod midi;
mod synth;
mod wavetable;

use crate::midi::{MidiEvent, MidiEventKind, MidiNote};
use crate::synth::Synth;

const TARGET_FPS: f32 = 200.0;
const FRAME_LEN: Duration = Duration::from_nanos((1_000_000_000f32 / TARGET_FPS) as u64);

pub fn main() -> Result<(), String> {
    let mut synth = Synth::new();
    let mut pressed_keys: HashSet<Keycode> = HashSet::new();

    let rendering_ctx = RenderingContext::new();
    let mut canvas = rendering_ctx.make_canvas();
    canvas.set_draw_color(Color::RGB(161, 88, 255)); // purple background

    let mut event_pump = rendering_ctx.sdl_context.event_pump()?;
    'running: loop {
        let frame_start = Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match event {
                    Event::KeyDown { .. } => {
                        if let Some(note) = keymap(keycode) {
                            if !pressed_keys.contains(&keycode) {
                                synth.send_midi_event(MidiEvent::new(note, MidiEventKind::NoteOn));
                                pressed_keys.insert(keycode);
                            }
                        }
                    }
                    Event::KeyUp { .. } => {
                        if let Some(note) = keymap(keycode) {
                            if pressed_keys.contains(&keycode) {
                                synth.send_midi_event(MidiEvent::new(note, MidiEventKind::NoteOff));
                                pressed_keys.remove(&keycode);
                            }
                        }
                    }
                    _ => unreachable!(),
                },
                _ => {}
            }
        }

        canvas.clear();
        canvas.present();

        std::thread::sleep(FRAME_LEN.saturating_sub(frame_start.elapsed()));
    }

    Ok(())
}

fn keymap(keycode: Keycode) -> Option<MidiNote> {
    let c4 = MidiNote::c0().octave_up(4);
    match keycode {
        // second row is white keys
        Keycode::A => Some(c4),
        Keycode::S => Some(c4.offset_up(2)),
        Keycode::D => Some(c4.offset_up(4)),
        Keycode::F => Some(c4.offset_up(5)),
        Keycode::G => Some(c4.offset_up(7)),
        Keycode::H => Some(c4.offset_up(9)),
        Keycode::J => Some(c4.offset_up(11)),
        Keycode::K => Some(c4.offset_up(12)),
        Keycode::L => Some(c4.offset_up(14)),
        Keycode::SEMICOLON => Some(c4.offset_up(16)),
        Keycode::QUOTE => Some(c4.offset_up(17)),
        // first row is black keys
        Keycode::W => Some(c4.offset_up(1)),
        Keycode::E => Some(c4.offset_up(3)),
        Keycode::T => Some(c4.offset_up(6)),
        Keycode::Y => Some(c4.offset_up(8)),
        Keycode::U => Some(c4.offset_up(10)),
        Keycode::I => Some(c4.offset_up(13)),
        Keycode::O => Some(c4.offset_up(15)),
        _ => None,
    }
}

struct RenderingContext {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
}

impl RenderingContext {
    fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        Self {
            sdl_context,
            video_subsystem,
        }
    }

    fn make_canvas(&self) -> sdl2::render::WindowCanvas {
        let window = self
            .video_subsystem
            .window("decapode", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .unwrap()
    }
}
