#![allow(unused)]

mod math;
mod synth;

use crate::synth::Synth;

fn main() {
    let mut synth = Synth::new();

    synth.set_frequency(256.0);
    std::thread::sleep(std::time::Duration::from_secs(1));
    synth.set_frequency(440.0);
    std::thread::sleep(std::time::Duration::from_secs(1));
    synth.set_frequency(256.0);
    std::thread::sleep(std::time::Duration::from_secs(1));

    /*
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    */
}
