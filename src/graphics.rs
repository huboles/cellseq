mod actions;
mod area;
mod keys;
mod layout;
mod map;
mod point;
mod selector;

pub use actions::*;
pub use area::*;
pub use keys::*;
pub use layout::*;
pub use map::*;
pub use point::*;
pub use selector::*;

use super::*;

use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event},
    execute, queue,
    style::{Print, SetAttributes, SetColors},
    terminal,
};
use eyre::Result;
use std::{
    io::{stdout, Write},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub fn render_loop(state: &mut GlobalState) -> Result<()> {
    execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;

    state.layout.draw_outlines()?;

    let world = Arc::new(Mutex::new(state.world.clone()));
    let state_arc = Arc::new(Mutex::new(state));

    loop {
        let arc = Arc::clone(&state_arc);
        let tick = arc.lock().unwrap().tick();
        let timer = std::thread::spawn(move || thread::sleep(tick));

        let event = std::thread::spawn(move || {
            if poll(tick).unwrap() {
                match read().unwrap() {
                    Event::Key(key) => key_event(key),
                    _ => Action::None,
                }
            } else {
                Action::None
            }
        });

        let mut maps = std::thread::spawn(|| {});
        if arc.lock().unwrap().transport.running {
            let map = world.clone();
            let mut mask = arc.lock().unwrap().mask.clone();
            let cell_area = arc.lock().unwrap().layout.cells;
            let mask_area = arc.lock().unwrap().layout.mask;

            maps = std::thread::spawn(move || {
                let mut map = map.lock().unwrap();
                map.update();
                let tmp = map.clone();
                draw_map(&tmp, &cell_area).unwrap();
                mask[0].update();
                let tmp = mask[0].clone();
                draw_map(&tmp, &mask_area).unwrap();
            });
        }

        let area = arc.lock().unwrap().layout.transport;

        arc.lock().unwrap().transport.render(area)?;

        action(event.join().unwrap(), arc.clone())?;
        maps.join().unwrap();

        arc.lock().unwrap().cursor.render()?;

        timer.join().unwrap();
        stdout().flush()?;
    }
}

pub fn draw_map<T>(map: &impl Map<T>, area: &Area) -> Result<()> {
    let ((x_zero, y_zero), (x_max, y_max)) = area.to_u16()?;

    let origin = Point::new(x_zero.into(), y_zero.into());

    let (char_on, char_off) = map.characters();
    let (on_colors, off_colors) = map.colors();
    let (style_on, style_off) = map.styles();

    for x in 0..=(map.x_size()) {
        for y in 0..=(map.y_size()) {
            let point = Point::new(x, y);
            let (x_off, y_off) = origin.u16_offset(point)?;

            if x_off <= x_zero || x_off >= x_max || y_off <= y_zero || y_off >= y_max - 1 {
                continue;
            }

            if map.try_point(point) {
                queue!(
                    stdout(),
                    MoveTo(x_off, y_off),
                    SetAttributes(style_on),
                    SetColors(on_colors),
                    Print(char_on)
                )?;
            } else {
                queue!(
                    stdout(),
                    MoveTo(x_off, y_off),
                    SetAttributes(style_off),
                    SetColors(off_colors),
                    Print(char_off)
                )?;
            }
        }
    }

    area.outline_area()?;

    Ok(())
}

pub fn loop_map<T>(
    map: &(impl Map<T> + Clone),
    area: &Area,
    time: Duration,
    steps: usize,
) -> Result<()> {
    loop {
        let mut tmp = map.clone();
        for _ in 0..steps {
            execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
            draw_map(&tmp, area)?;
            stdout().flush()?;
            tmp.update();
            thread::sleep(time);
        }
    }
}
