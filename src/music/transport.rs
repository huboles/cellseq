use super::*;

pub struct Song {
    pub key: Option<Note>,
    pub scale: Option<Scale>,
}

impl Song {
    pub fn new(key: Option<Note>, scale: Option<Scale>) -> Self {
        Self { key, scale }
    }
}
