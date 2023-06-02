use crate::Point;
use crossterm::{
    cursor, queue,
    style::{self, Attribute, Attributes, Color, Colors},
};
use eyre::Result;
use std::{
    io::stdout,
    ops::{Deref, DerefMut},
};

use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub position: Point,
    pub color: Colors,
    pub style: Attributes,
    pub char: char,
    pub area: Area,
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

impl Default for Cursor {
    fn default() -> Self {
        Self::new(Area::new(0, 0, 0, 0))
    }
}

impl Cursor {
    pub fn new(area: Area) -> Self {
        Self {
            position: Point::new(1, 1),
            color: Colors::new(Color::White, Color::Black),
            char: '█',
            style: Attributes::from(Attribute::Bold),
            area,
        }
    }

    pub fn render(&self) -> Result<()> {
        let offset = self.area.origin + self.position;
        queue!(
            stdout(),
            cursor::Hide,
            cursor::MoveTo(offset.x.try_into().unwrap(), offset.y.try_into().unwrap()),
            style::SetAttributes(self.style),
            style::SetColors(self.color),
            cursor::Show,
            style::Print(self.char.to_string())
        )?;
        Ok(())
    }
}
