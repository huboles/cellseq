use cellseq::*;
use eyre::Result;

fn main() -> Result<()> {
    let mut map = Map::new(48, 24);
    map.randomize(0.75);

    run_map(&mut map, (10, 5), 300)?;
    Ok(())
}
