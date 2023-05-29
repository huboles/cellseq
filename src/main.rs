use cellseq::*;
use eyre::Result;

use crossterm::{
    event::{poll, read, Event},
    terminal,
};

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;

    action_loop(10)?;
    // let mut mask: Mask<bool> = Mask::new(48, 24, false);
    //
    // let mut map = World::new(74, 32);
    // map.randomize(0.75);

    // let time = bpm_to_ms(100);

    // loop {
    //     if poll(std::time::Duration::from_millis(50))? {
    //         // It's guaranteed that the `read()` won't block when the `poll()`
    //         // function returns `true`
    //         match read()? {
    //             Event::Key(event) => println!("\n{:?}", event),
    //             _ => continue,
    //         }
    //     }
    // }

    // run_map(&mut map, Point::new(10, 2), time)?;
    // edit_mask(&mask, (10, 5))?;
    // move_cursor()?;
    Ok(())
}
