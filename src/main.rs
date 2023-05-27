use cellseq::*;
use eyre::Result;

fn main() -> Result<()> {
    let mut mask: Mask<bool> = Mask::new(48, 24);
    let mut map = Map::new(48, 24);
    map.randomize(0.75);

    // loop_map(&mut map, (10, 5), std::time::Duration::from_millis(250), 32)?;
    // draw_mask::<bool>(&mask, (10, 5))?;
    edit_mask(&mask, (10, 5))?;
    Ok(())
}
