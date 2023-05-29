use crate::Point;
use crossterm::{
    cursor, queue,
    style::{self, Attribute, Color},
};
use eyre::Result;
use std::{
    io::stdout,
    ops::{Deref, DerefMut},
};

pub struct Cursor {
    pub position: Point,
    pub fg: Color,
    pub bg: Color,
    pub style: Attribute,
    pub char: char,
}

impl Deref for Cursor {
    type Target = Point;
    fn deref(&self) -> &Self::Target {
        &self.position
    }
}

impl DerefMut for Cursor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.position
    }
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            position: Point::new(0, 0),
            fg: Color::White,
            bg: Color::Black,
            char: '*',
            style: Attribute::Reset,
        }
    }

    pub fn render(&self) -> Result<()> {
        queue!(
            stdout(),
            cursor::Hide,
            cursor::MoveTo(self.x.try_into().unwrap(), self.y.try_into().unwrap()),
            style::SetAttribute(self.style),
            style::SetForegroundColor(self.fg),
            style::SetBackgroundColor(self.bg),
            style::Print(self.char.to_string())
        )?;
        Ok(())
    }
}
