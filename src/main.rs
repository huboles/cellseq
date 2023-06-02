use cellseq::*;
use crossterm::{cursor, execute, terminal};
use eyre::Result;

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    execute!(std::io::stdout(), cursor::Hide)?;

    let mut state = GlobalState::build()?;

    state.world.randomize(0.75);
    state.mask[0].randomize(0.75, Scale::Aeolian);

    match render_loop(&mut state) {
        Ok(_) => exit(),
        Err(e) => {
            eprintln!("{}", e);
            exit()
        }
    }
}
