use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::*;
use crate::Clock;

pub fn key_event(event: KeyEvent) -> Action {
    match event.modifiers {
        KeyModifiers::NONE => match_normal_key(event.code),
        KeyModifiers::CONTROL => match_ctl_key(event.code),
        KeyModifiers::SHIFT => match_shift_key(event.code),
        _ => Action::None,
    }
}

pub fn match_normal_key(key: KeyCode) -> Action {
    match key {
        KeyCode::Enter => Action::Clock(Clock::Pause),
        KeyCode::Esc => Action::Exit,
        KeyCode::Tab => Action::SwitchPanel,
        KeyCode::Up => Action::Move(Direction::Up),
        KeyCode::Down => Action::Move(Direction::Down),
        KeyCode::Right => Action::Move(Direction::Right),
        KeyCode::Left => Action::Move(Direction::Left),
        KeyCode::Char(c) => match c {
            '?' => Action::Help,
            ' ' => Action::Select,
            '[' => Action::Clock(Clock::Slower(1)),
            ']' => Action::Clock(Clock::Faster(1)),
            '{' => Action::Clock(Clock::Slower(5)),
            '}' => Action::Clock(Clock::Faster(5)),
            'q' => Action::Exit,
            'r' => Action::Reload,
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
        KeyCode::Char('c') => Action::Exit,
        _ => Action::None,
    }
}

pub fn match_shift_key(key: KeyCode) -> Action {
    match key {
        KeyCode::Enter => Action::Clock(Clock::Stop),
        KeyCode::Char(c) => match c {
            ' ' => Action::SelectArea,
            'r' => Action::Randomize,
            _ => Action::None,
        },
        _ => Action::None,
    }
}
