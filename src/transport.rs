use super::*;

pub struct Transport {
    pub running: bool,
    pub bpm: usize,
    pub sig: TimeSignature,
    pub repeat: usize,
}

impl Transport {
    pub fn new(top: usize, bottom: usize, bpm: usize) -> Self {
        Self {
            sig: TimeSignature { top, bottom },
            bpm,
            running: true,
            repeat: 0,
        }
    }
}
