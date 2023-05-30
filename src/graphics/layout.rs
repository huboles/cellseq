use crossterm::terminal;
use eyre::Result;

use super::*;

pub struct Area {
    pub origin: Point,
    pub max: Point,
}

pub struct Layout {
    pub screen: Area,
    pub cells: Area,
    pub mask: Area,
    pub channels: Area,
}

impl Layout {
    pub fn build() -> Result<Self> {
        let (col, row) = terminal::size()?;
        let col: usize = col.into();
        let row: usize = row.into();

        let screen = Area {
            origin: Point::new(0, 0),
            max: Point::new(col, row),
        };

        let cells = Area {
            origin: Point::new(1, 1),
            max: Point::new(col / 2 - 5, row / 2 - 2),
        };

        let mask = Area {
            origin: Point::new(col / 2 + 5, 1),
            max: Point::new(col - 2, row / 2 - 2),
        };

        let channels = Area {
            origin: Point::new(1, row / 2 + 2),
            max: Point::new(col - 2, row - 2),
        };

        Ok(Self {
            screen,
            cells,
            mask,
            channels,
        })
    }
}
