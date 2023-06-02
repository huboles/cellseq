mod transport;

pub use transport::*;

use super::*;

pub struct GlobalState {
    pub world: World,
    pub layout: Layout,
    pub transport: Transport,
    pub song: Song,
    pub mask: Vec<Mask>,
    pub channels: Vec<MidiChannel>,
    pub cursor: Cursor,
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

        Ok(Self {
            world,
            layout,
            transport,
            song,
            mask,
            channels,
            cursor,
        })
    }

    pub fn tick(&self) -> Duration {
        bpm_to_ms(self.transport.bpm)
    }
}

pub struct MidiChannel {
    pub num: usize,
    pub poly_num: usize,
}
