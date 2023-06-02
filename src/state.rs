mod transport;

pub use transport::*;

use crossbeam::channel::{bounded, Receiver, Sender};

use super::*;

#[derive(Debug, Clone)]
pub struct GlobalState {
    pub world: World,
    pub layout: Layout,
    pub transport: Transport,
    pub song: Song,
    pub mask: Vec<Mask>,
    pub channels: (usize, Vec<MidiChannel>),
    pub cursor: Cursor,
    pub update_mask: (Sender<usize>, Receiver<usize>),
}

impl GlobalState {
    pub fn build() -> Result<Self> {
        let layout = Layout::build()?;

        let world = World::new(layout.cells);

        let channels = Vec::with_capacity(10);
        let mut mask: Vec<Mask> = Vec::with_capacity(10);

        for _ in 0..10 {
            mask.push(Mask::new(layout.mask));
        }

        let transport = Transport::new(4, 4, 120);

        let cursor = Cursor::new(layout.mask);

        let song = Song::new(None, None);

        let update_mask = bounded::<usize>(0);

        Ok(Self {
            world,
            layout,
            transport,
            song,
            mask,
            channels: (0, channels),
            cursor,
            update_mask,
        })
    }

    pub fn tick(&self) -> Duration {
        bpm_to_ms(self.transport.bpm)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MidiChannel {
    pub num: usize,
    pub poly_num: usize,
}
