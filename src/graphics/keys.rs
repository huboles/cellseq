use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::*;

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
        KeyCode::Char(c) => match c {
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
