use super::*;
use crossterm::event::{poll, read, Event};

#[derive(Debug)]
pub enum Action {
    None,
    Move(Direction),
    Transport(Clock),
    Channel(usize),
    Select,
    SelectArea,
    Reload,
    Randomize,
    Exit,
    Help,
    Edit,
}

#[derive(Debug)]
pub enum Clock {
    Stop,
    Start,
    Pause,
    Faster(usize),
    Slower(usize),
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn action_loop(speed: usize) -> Result<()> {
    loop {
        if poll(Duration::from_millis(speed.try_into()?))? {
            if let Event::Key(key) = read()? {
                match key_event(key) {
                    Action::None => continue,
                    Action::Exit => crate::exit()?,
                    Action::Move(_direction) => todo!(),
                    Action::Channel(_channel) => todo!(),
                    Action::Transport(_clock) => todo!(),
                    Action::Edit => todo!(),
                    Action::Select => todo!(),
                    Action::SelectArea => todo!(),
                    Action::Reload => todo!(),
                    Action::Randomize => todo!(),
                    Action::Help => todo!(),
                }
            }
        }
    }
}
