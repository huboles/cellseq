mod map;
mod point;
mod selector;

pub use map::*;
pub use point::*;
pub use selector::*;

use crossterm::{cursor, execute, queue, style, terminal};
use eyre::Result;
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

pub struct Cursor {}

pub fn draw_frame<T>(map: &mut impl Map<T>, offset: Point) -> Result<()> {
    let (char_on, char_off) = map.characters();
    let (fg_on, fg_off) = map.fg_colors();
    let (bg_on, bg_off) = map.bg_colors();
    let (style_on, style_off) = map.styles();

    for x in 1..(map.x_size() - 1) {
        for y in 1..(map.y_size() - 1) {
            let point = Point::new(x, y);
            let (x_off, y_off) = point.u16_offset(offset)?;

            if map.try_point(point) {
                queue!(
                    stdout(),
                    cursor::Hide,
                    cursor::MoveTo(x_off, y_off),
                    style::SetAttribute(style_on),
                    style::SetForegroundColor(fg_on),
                    style::SetBackgroundColor(bg_on),
                    style::Print(char_on)
                )?;
            } else {
                queue!(
                    stdout(),
                    cursor::Hide,
                    cursor::MoveTo(x_off, y_off),
                    style::SetAttribute(style_off),
                    style::SetForegroundColor(fg_off),
                    style::SetBackgroundColor(bg_off),
                    style::Print(char_off)
                )?;
            }
        }
    }
    stdout().flush()?;
    Ok(())
}

pub fn run_map<T>(map: &mut impl Map<T>, offset: Point, time: Duration) -> Result<()> {
    loop {
        map.update();

        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;

        draw_frame(map, offset)?;

        thread::sleep(time);
    }
}

pub fn loop_map<T>(
    map: &mut (impl Map<T> + Clone),
    offset: Point,
    time: Duration,
    steps: usize,
) -> Result<()> {
    loop {
        let mut tmp = map.clone();
        for _ in 0..steps {
            execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
            draw_frame(&mut tmp, offset)?;
            tmp.update();
            thread::sleep(time);
        }
    }
}
