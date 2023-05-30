use cellseq::*;
use crossterm::terminal;
use eyre::Result;

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;

    let mut state = GlobalState::build()?;

    state.world.randomize(0.75);

    match render_loop(&mut state) {
        Ok(_) => exit(),
        Err(e) => {
            eprintln!("{}", e);
            exit()
        }
    }
}
