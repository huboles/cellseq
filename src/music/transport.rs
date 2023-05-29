use super::*;

pub struct Song {
    pub bpm: usize,
    pub time_sig: TimeSignature,
    pub key: Option<Note>,
    pub scale: Option<Scale>,
}

impl Song {
    pub fn new(
        bpm: usize,
        time_sig: TimeSignature,
        key: Option<Note>,
        scale: Option<Scale>,
    ) -> Self {
        Self {
            bpm,
            time_sig,
            key,
            scale,
        }
    }
}

pub enum Transport {
    Pause,
    Play,
    FastForward(usize),
}
