use std::time::Duration;

pub struct Envelope {
    attack: Duration,
    release: Duration,
}

impl Envelope {
    pub fn new(attack: Duration, release: Duration) -> Self {
        Self { attack, release }
    }
}
