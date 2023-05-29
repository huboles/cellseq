use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue, terminal,
};
use eyre::Result;
use std::time::Duration;

pub fn action_loop(speed: usize) -> Result<()> {
    loop {
        if poll(Duration::from_millis(speed.try_into()?))? {
            if let Event::Key(key) = read()? {
                match key_event(key) {
                    Action::None => continue,
                    Action::Exit => crate::exit()?,
                    x => println!("{:?}", x),
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum Action {
    None,
    Move(Direction),
    Resize(Direction),
    Transport(Clock),
    Channel(usize),
    Select,
    SelectArea,
    Reload,
    Randomize,
    Exit,
    Help,
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

pub fn key_event(event: KeyEvent) -> Action {
    match event.modifiers {
        KeyModifiers::NONE => match_normal_key(event.code),
        KeyModifiers::CONTROL => match_ctl_key(event.code),
        KeyModifiers::ALT => match_alt_key(event.code),
        KeyModifiers::SHIFT => match_shift_key(event.code),
        _ => Action::None,
    }
}

pub fn match_normal_key(key: KeyCode) -> Action {
    match key {
        KeyCode::Enter => Action::Transport(Clock::Pause),
        KeyCode::Esc => Action::Exit,
        KeyCode::Up => Action::Move(Direction::Up),
        KeyCode::Down => Action::Move(Direction::Down),
        KeyCode::Right => Action::Move(Direction::Right),
        KeyCode::Left => Action::Move(Direction::Left),
        KeyCode::Char(c) => match c {
            '?' => Action::Help,
            ' ' => Action::Select,
            'q' => Action::Exit,
            'j' => Action::Move(Direction::Down),
            'k' => Action::Move(Direction::Up),
            'h' => Action::Move(Direction::Left),
            'l' => Action::Move(Direction::Right),
            'r' => Action::Reload,
            '[' => Action::Transport(Clock::Slower(1)),
            ']' => Action::Transport(Clock::Faster(1)),
            '{' => Action::Transport(Clock::Slower(5)),
            '}' => Action::Transport(Clock::Faster(5)),
            '0' => Action::Channel(0),
            '1' => Action::Channel(1),
            '2' => Action::Channel(2),
            '3' => Action::Channel(3),
            '4' => Action::Channel(4),
            '5' => Action::Channel(5),
            '6' => Action::Channel(6),
            '7' => Action::Channel(7),
            '8' => Action::Channel(8),
            '9' => Action::Channel(9),
            _ => Action::None,
        },
        _ => Action::None,
    }
}

pub fn match_ctl_key(key: KeyCode) -> Action {
    match key {
        KeyCode::Char(c) => match c {
            'c' => Action::Exit,
            _ => Action::None,
        },
        _ => Action::None,
    }
}

pub fn match_shift_key(key: KeyCode) -> Action {
    match key {
        KeyCode::Up => Action::Resize(Direction::Up),
        KeyCode::Down => Action::Resize(Direction::Down),
        KeyCode::Right => Action::Resize(Direction::Right),
        KeyCode::Left => Action::Resize(Direction::Left),
        KeyCode::Char(c) => match c {
            'J' => Action::Resize(Direction::Down),
            'K' => Action::Resize(Direction::Up),
            'H' => Action::Resize(Direction::Left),
            'L' => Action::Resize(Direction::Right),
            'R' => Action::Randomize,
            _ => Action::None,
        },
        _ => Action::None,
    }
}

pub fn match_alt_key(key: KeyCode) -> Action {
    match key {
        KeyCode::Char(c) => match c {
            _ => Action::None,
        },
        _ => Action::None,
    }
}
