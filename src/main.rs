use cellseq::*;
use crossterm::{cursor::Hide, execute, terminal};
use eyre::Result;
use std::io::stdout;

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout(), Hide)?;

    let mut state = GlobalState::build()?;

    state.world.randomize(0.75);
    state.mask[0].randomize(0.75, Scale::Aeolian);

    match main_loop(state) {
        Ok(_) => exit(),
        Err(e) => {
            eprintln!("{}", e);
            exit()
        }
    }
}
