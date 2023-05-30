use crossterm::terminal;
use eyre::Result;

use super::*;

#[derive(Clone, Debug)]
pub struct Layout {
    pub screen: Area,
    pub cells: Area,
    pub mask: Area,
    pub channels: Area,
    pub transport: Area,
}

impl Layout {
    pub fn build() -> Result<Self> {
        let (col, row) = terminal::size()?;

        let col: usize = col.into();
        let row: usize = row.into();

        let screen = Area::new(0, 0, col, row);
        let cells = Area::new(1, 1, col / 2 - 5, row / 2 - 2);
        let mask = Area::new(col / 2 + 5, 1, col - 2, row / 2 - 2);
        let channels = Area::new(1, row / 2 + 2, col - 2, row - 2);
        let transport = Area::new(col / 2 - 4, 4, col / 2 + 4, 8);

        Ok(Self {
            screen,
            cells,
            mask,
            channels,
            transport,
        })
    }

    pub fn draw_outlines(&self) -> Result<()> {
        self.cells.outline_area()?;
        self.mask.outline_area()?;
        self.channels.outline_area()?;
        self.transport.outline_area()?;
        Ok(())
    }
}
