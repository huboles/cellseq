use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue, terminal,
};

use super::*;

pub enum Action {
    None,
    Move(Direction),
    Resize(Direction),
    Transport(Transport),
    Select,
    UnSelect,
    Exit,
}

pub enum Transport {
    Stop,
    Start,
    Pause,
    Faster(usize),
    Slower(usize),
}

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
        KeyCode::Enter => Action::Transport(Transport::Pause),
        KeyCode::Esc => Action::Exit,
        KeyCode::Up => Action::Move(Direction::Up),
        KeyCode::Down => Action::Move(Direction::Down),
        KeyCode::Right => Action::Move(Direction::Right),
        KeyCode::Left => Action::Move(Direction::Left),
        KeyCode::Char(c) => match c {
            ' ' => Action::Select,
            'q' => Action::Exit,
            'j' => Action::Move(Direction::Down),
            'k' => Action::Move(Direction::Up),
            'h' => Action::Move(Direction::Left),
            'l' => Action::Move(Direction::Right),
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
        KeyCode::Enter => Action::Transport(Transport::Stop),
        KeyCode::Up => Action::Resize(Direction::Up),
        KeyCode::Down => Action::Resize(Direction::Down),
        KeyCode::Right => Action::Resize(Direction::Right),
        KeyCode::Left => Action::Resize(Direction::Left),
        KeyCode::Char(c) => match c {
            ' ' => Action::UnSelect,
            'j' => Action::Resize(Direction::Down),
            'k' => Action::Resize(Direction::Up),
            'h' => Action::Resize(Direction::Left),
            'l' => Action::Resize(Direction::Right),
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
