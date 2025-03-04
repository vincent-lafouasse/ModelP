#![allow(unused)]

use std::time::{Duration, Instant};

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

mod math;
mod synth;

use crate::synth::Synth;

const TARGET_FPS: f32 = 10.0;
const FRAME_LEN: Duration = Duration::from_nanos((1_000_000_000f32 / TARGET_FPS) as u64);

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let mut synth = Synth::new();

    'running: loop {
        let frame_start = Instant::now();
        for event in event_pump.poll_iter() {
            if let Event::KeyDown {
                keycode: Some(keycode),
                ..
            } = event
            {
                match keycode {
                    Keycode::A => synth.set_frequency(256.0),
                    Keycode::G => synth.set_frequency(256.0 * 1.5),
                    Keycode::Space => synth.toggle_playback(),
                    _ => {}
                }
            }
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.clear();
        canvas.present();

        ::std::thread::sleep(FRAME_LEN.saturating_sub(frame_start.elapsed()));
    }

    Ok(())
}
