use std::{io::stdout, time::Duration};

use super::*;

use crossterm::{
    cursor::Show,
    event::{poll, read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, Clear, ClearType::All},
};
use tokio::sync::mpsc::{self, Receiver, Sender};

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Exit,
    Help,
    MoveCursor(Direction),
    SwapSide,
    Remove,
    Select,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub async fn run_keys(snd: Sender<Action>) -> Result<()> {
    loop {
        if poll(Duration::from_millis(100)).unwrap() {
            match read().unwrap() {
                Event::Resize(_x, _y) => todo!(),
                Event::Key(k) => match k.code {
                    KeyCode::Backspace => todo!(),
                    KeyCode::Enter => todo!(),
                    KeyCode::Tab => snd.send(Action::SwapSide).await?,
                    KeyCode::Esc => snd.send(Action::Exit).await?,
                    KeyCode::Up => snd.send(Action::MoveCursor(Direction::Up)).await?,
                    KeyCode::Down => snd.send(Action::MoveCursor(Direction::Down)).await?,
                    KeyCode::Left => snd.send(Action::MoveCursor(Direction::Left)).await?,
                    KeyCode::Right => snd.send(Action::MoveCursor(Direction::Right)).await?,
                    KeyCode::Char(c) => match c {
                        ' ' => snd.send(Action::Select).await?,
                        'q' => snd.send(Action::Exit).await?,
                        '?' => snd.send(Action::Help).await?,
                        'h' => snd.send(Action::MoveCursor(Direction::Left)).await?,
                        'j' => snd.send(Action::MoveCursor(Direction::Down)).await?,
                        'k' => snd.send(Action::MoveCursor(Direction::Up)).await?,
                        'l' => snd.send(Action::MoveCursor(Direction::Right)).await?,
                        _ => continue,
                    },
                    _ => continue,
                },
                _ => continue,
            }
        }
    }
}

pub async fn run_action(rcv: &mut Receiver<Action>) -> Result<()> {
    while let Some(action) = rcv.recv().await {
        match action {
            Action::Exit => exit(),
            Action::Help => todo!(),
            Action::MoveCursor(_) => todo!(),
            Action::SwapSide => todo!(),
            Action::Remove => todo!(),
            Action::Select => todo!(),
        }
    }

    Ok(())
}

fn exit() {
    disable_raw_mode().unwrap();
    execute!(stdout(), Clear(All), Show).unwrap();

    std::process::exit(0);
}
