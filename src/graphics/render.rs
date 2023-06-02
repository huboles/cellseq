use super::*;

use crossbeam::channel::{bounded, Receiver, Sender};

pub type ArcState = Arc<Mutex<GlobalState>>;

pub fn main_loop(state: GlobalState) -> Result<()> {
    execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;

    state.layout.draw_outlines()?;

    let (clock, sync) = bounded::<()>(0);
    let state = Arc::new(Mutex::new(state));

    let timer_state = Arc::clone(&state);
    let world_state = Arc::clone(&state);
    let action_state = Arc::clone(&state);
    // let cursor_state = Arc::clone(&state);

    let timer = std::thread::spawn(move || timer_loop(timer_state, clock));
    let world = std::thread::spawn(move || world_loop(world_state, sync));
    let action = std::thread::spawn(move || action_loop(action_state));
    // let cursor = std::thread::spawn(move || cursor_loop(cursor_state));

    timer.join().unwrap()?;
    world.join().unwrap()?;
    action.join().unwrap()?;
    // cursor.join().unwrap()?;

    Ok(())
}

fn timer_loop(state: ArcState, clock: Sender<()>) -> Result<()> {
    loop {
        let tick = state.lock().unwrap().tick();
        thread::sleep(tick);
        clock.send(())?;
    }
}

pub fn world_loop(state: ArcState, sync: Receiver<()>) -> Result<()> {
    loop {
        let mutex = state.lock().unwrap();
        if mutex.transport.running {
            let mut world = mutex.world.clone();
            let mask = mutex.mask[mutex.channels.0].clone();
            let world_layout = mutex.layout.cells;
            let mask_layout = mutex.layout.mask;
            drop(mutex);

            world.update();
            draw_map(&world, &world_layout)?;
            draw_map(&mask, &mask_layout)?;
            sync.recv()?;
            state.lock().as_mut().unwrap().world = world;
        } else {
            drop(mutex);
            sync.recv()?;
        }
    }
}

pub fn action_loop(state: ArcState) -> Result<()> {
    while let Ok(event) = read() {
        match event {
            Event::Key(key) => action(key_event(key), state.clone())?,
            Event::Resize(_, _) => state.lock().unwrap().layout = Layout::build()?,
            _ => (),
        }
    }
    Ok(())
}

pub fn cursor_loop(state: ArcState) -> Result<()> {
    loop {
        state.lock().unwrap().cursor.render()?;
        std::thread::sleep(Duration::from_millis(100))
    }
}

// pub fn render_loop(state: &mut GlobalState) -> Result<()> {
//     execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;

//     state.layout.draw_outlines()?;

//     let world = Arc::new(Mutex::new(state.world.clone()));
//     let state_arc = Arc::new(Mutex::new(state));

//     loop {
//         let arc = Arc::clone(&state_arc);
//         let tick = arc.lock().unwrap().tick();
//         let timer = std::thread::spawn(move || thread::sleep(tick));

//         let event = std::thread::spawn(move || {
//             if poll(tick).unwrap() {
//                 match read().unwrap() {
//                     Event::Key(key) => key_event(key),
//                     _ => Action::None,
//                 }
//             } else {
//                 Action::None
//             }
//         });

//         let mut maps = std::thread::spawn(|| {});
//         if arc.lock().unwrap().transport.running {
//             let map = world.clone();
//             let mut mask = arc.lock().unwrap().mask.clone();
//             let cell_area = arc.lock().unwrap().layout.cells;
//             let mask_area = arc.lock().unwrap().layout.mask;

//             maps = std::thread::spawn(move || {
//                 let mut map = map.lock().unwrap();
//                 map.update();
//                 let tmp = map.clone();
//                 draw_map(&tmp, &cell_area).unwrap();
//                 mask[0].update();
//                 let tmp = mask[0].clone();
//                 draw_map(&tmp, &mask_area).unwrap();
//             });
//         }

//         let area = arc.lock().unwrap().layout.transport;

//         arc.lock().unwrap().transport.render(area)?;

//         action(event.join().unwrap(), arc.clone())?;
//         maps.join().unwrap();

//         arc.lock().unwrap().cursor.render()?;

//         timer.join().unwrap();
//         stdout().flush()?;
//     }
// }
