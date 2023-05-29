use cellseq::*;
use eyre::Result;

fn main() -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;

    let mut mask: Mask<bool> = Mask::new(48, 24, false);
    let mut map = World::new(74, 32);
    map.randomize(0.75);

    let time = bpm_to_ms(100);

    run_map(&mut map, Point::new(10, 2), time)?;
    // edit_mask(&mask, (10, 5))?;
    // move_cursor()?;
    Ok(())
}
