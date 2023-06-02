use super::*;

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Attribute, Color, Colors, Print, SetAttributes, SetColors},
};
use std::io::stdout;

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

    pub fn render(&self, area: Area) -> Result<()> {
        let ((x_zero, y_zero), (_, _)) = area.to_u16()?;

        let bpm = format!("bpm: {}", self.bpm);
        let sig = format!("sig: {}/{}", self.sig.top, self.sig.bottom);
        let rep = format!("rep: {}", self.repeat);

        queue!(
            stdout(),
            MoveTo(x_zero + 1, y_zero + 1),
            SetAttributes(Attribute::Bold.into()),
            SetColors(Colors::new(Color::White, Color::Black)),
            Print(bpm),
            MoveTo(x_zero + 1, y_zero + 2),
            SetAttributes(Attribute::Bold.into()),
            SetColors(Colors::new(Color::White, Color::Black)),
            Print(sig),
            MoveTo(x_zero + 1, y_zero + 3),
            SetAttributes(Attribute::Bold.into()),
            SetColors(Colors::new(Color::White, Color::Black)),
            Print(rep),
        )?;

        Ok(())
    }
}
