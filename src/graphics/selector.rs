use crate::Point;
use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{self, Attribute, Color},
    terminal,
};
use eyre::Result;
use std::{
    io::stdout,
    ops::{Deref, DerefMut},
    time::Duration,
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

    pub fn update(&mut self, event: KeyEvent) -> Result<()> {
        let key = event.code;
        match event.modifiers {
            KeyModifiers::NONE => match key {
                KeyCode::Char(c) => match c {
                    'j' => self.y += 1,
                    'k' => {
                        if self.y != 0 {
                            self.y -= 1
                        }
                    }
                    'h' => {
                        if self.x != 0 {
                            self.x -= 1
                        }
                    }
                    'l' => self.x += 1,
                    _ => {}
                },
                _ => {}
            },
            KeyModifiers::CONTROL => match key {
                KeyCode::Char(c) => match c {
                    'c' => std::process::exit(0),
                    _ => {}
                },
                _ => {}
            },
            KeyModifiers::ALT => match key {
                _ => {}
            },
            KeyModifiers::SHIFT => match key {
                _ => {}
            },
            _ => {}
        };
        Ok(())
    }
}

pub fn move_cursor() -> Result<()> {
    let mut cursor = Cursor::new();
    loop {
        if poll(Duration::from_millis(10))? {
            execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
            match read()? {
                Event::Key(event) => {
                    cursor.update(event)?;
                }
                _ => continue,
            }
        }
        cursor.render()?;
    }
}
