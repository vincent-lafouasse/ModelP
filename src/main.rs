#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::collections::HashSet;

use egui::Key;

mod event;
mod math;
mod midi;
mod synth;
mod tuner;
mod wavetable;

use crate::event::Event;
use crate::midi::MidiNote;
use crate::synth::Synth;

struct App {
    synth: Synth,
    pressed_keys: HashSet<egui::Key>,
    root_note: MidiNote,
}

impl Default for App {
    fn default() -> Self {
        let synth = Synth::new();
        let pressed_keys: HashSet<egui::Key> = HashSet::new();
        let root_note = MidiNote::c(2);

        Self {
            synth,
            pressed_keys,
            root_note,
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "decapode",
        options,
        Box::new(|_cc| Ok(Box::<App>::default())),
    )
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ctx.input(|i| i.viewport().close_requested()) {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }

            let events = ui.ctx().input(|i| i.events.clone());
            'event_loop: for event in &events {
                if let egui::Event::Key {
                    key: Key::Escape,
                    pressed: false,
                    ..
                } = event
                {
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
                if let egui::Event::Key {
                    key: Key::Z,
                    pressed: false,
                    ..
                } = event
                {
                    self.synth.send_midi_event(Event::OctaveDown);
                } else if let egui::Event::Key {
                    key: Key::X,
                    pressed: false,
                    ..
                } = event
                {
                    self.synth.send_midi_event(Event::OctaveUp);
                }

                match event {
                    egui::Event::Key { key, pressed, .. } => {
                        let note = keymap(key, self.root_note);
                        if note.is_none() {
                            continue 'event_loop;
                        }
                        let note = note.unwrap();
                        match pressed {
                            // NoteOn
                            true => {
                                if !self.pressed_keys.contains(key) {
                                    self.synth.send_midi_event(Event::NoteOn(note));
                                    self.pressed_keys.insert(*key);
                                }
                            }
                            // NoteOff
                            false => {
                                if self.pressed_keys.contains(key) {
                                    self.synth.send_midi_event(Event::NoteOff(note));
                                    self.pressed_keys.remove(key);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        });
    }
}
fn keymap(keycode: &Key, root: MidiNote) -> Option<MidiNote> {
    match keycode {
        // second row is white keys
        Key::A => Some(root),
        Key::S => Some(root.offset_up(2)),
        Key::D => Some(root.offset_up(4)),
        Key::F => Some(root.offset_up(5)),
        Key::G => Some(root.offset_up(7)),
        Key::H => Some(root.offset_up(9)),
        Key::J => Some(root.offset_up(11)),
        Key::K => Some(root.offset_up(12)),
        Key::L => Some(root.offset_up(14)),
        Key::Semicolon => Some(root.offset_up(16)),
        Key::Quote => Some(root.offset_up(17)),
        // first row is black keys
        Key::W => Some(root.offset_up(1)),
        Key::E => Some(root.offset_up(3)),
        Key::T => Some(root.offset_up(6)),
        Key::Y => Some(root.offset_up(8)),
        Key::U => Some(root.offset_up(10)),
        Key::I => Some(root.offset_up(13)),
        Key::O => Some(root.offset_up(15)),
        _ => None,
    }
}
