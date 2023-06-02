use super::*;

#[derive(Debug)]
pub enum Action {
    None,
    Move(Direction),
    Clock(Clock),
    Channel(usize),
    Select,
    SelectArea,
    SwitchPanel,
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

type AMmGS<'a> = Arc<Mutex<&'a mut GlobalState>>;

pub fn action(action: Action, state: AMmGS) -> Result<()> {
    match action {
        Action::None => Ok(()),
        Action::Exit => exit(),
        Action::Move(direction) => move_cursor(direction, state),
        Action::Clock(clock) => transport_action(clock, state),
        Action::Channel(_) => Ok(()),
        Action::Select => Ok(()),
        Action::SelectArea => Ok(()),
        Action::Reload => Ok(()),
        Action::Randomize => Ok(()),
        Action::Help => Ok(()),
        Action::Edit => Ok(()),
        Action::SwitchPanel => Ok(()),
    }
}

fn transport_action(clock: Clock, state: AMmGS) -> Result<()> {
    let mut state = state.lock().unwrap();
    match clock {
        Clock::Stop => state.transport.running = false,
        Clock::Start => state.transport.running = true,
        Clock::Pause => state.transport.running = !state.transport.running,
        Clock::Faster(n) => state.transport.bpm += n,
        Clock::Slower(n) => state.transport.bpm -= n,
    };
    Ok(())
}

fn move_cursor(direction: Direction, state: AMmGS) -> Result<()> {
    let mut state = state.lock().unwrap();
    match direction {
        Direction::Up => {
            if state.cursor.position.y > 1 {
                state.cursor.position.y -= 1
            }
        }
        Direction::Down => {
            if state.cursor.position.y < state.cursor.area.height() {
                state.cursor.position.y += 1
            }
        }
        Direction::Left => {
            if state.cursor.position.x > 1 {
                state.cursor.position.x -= 1
            }
        }
        Direction::Right => {
            if state.cursor.position.x < state.cursor.area.width() {
                state.cursor.position.x += 1
            }
        }
    }
    Ok(())
}
