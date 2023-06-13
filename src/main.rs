use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{enable_raw_mode, Clear, ClearType::All},
};
use eyre::Result;
use std::io::stdout;
use tokio::{
    sync::{mpsc, watch},
    try_join,
};

use cellseq::*;

#[tokio::main]
async fn main() -> Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), Clear(All), Hide)?;
    let (clock_snd, clock_rcv) = watch::channel(0);
    let (action_snd, mut action_rcv) = mpsc::channel::<Action>(16);

    let mut metro = Metronome::new(clock_snd, 120);
    let clock = run_clock(&mut metro);

    let cell_area = Area::new(Point::from((1, 1)), Point::from((60, 25)));
    cell_area.draw_outline()?;
    let mut map = World::new(cell_area);
    map.randomize(0.5);
    let mut world_clock = clock_rcv.clone();
    let world = run_world(&mut world_clock, &mut map);

    let input = run_keys(action_snd);
    let action = run_action(&mut action_rcv);

    if let Err(e) = try_join!(clock, world, input, action) {
        execute!(stdout(), Clear(All), Show)?;
        eprintln!("{e}");
        return Err(e);
    } else {
        Ok(())
    }
}
