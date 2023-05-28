use cellseq::*;
use eyre::Result;

fn main() -> Result<()> {
    let mut mask: Mask<bool> = Mask::new(48, 24, false);
    let mut map = World::new(48, 24);
    map.randomize(0.75);

    let time = bpm_to_ms(120);

    loop_map(&mut map, Point::new(10, 5), time, 32)?;
    // edit_mask(&mask, (10, 5))?;
    Ok(())
}
