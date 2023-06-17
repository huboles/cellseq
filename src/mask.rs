use super::*;

use rustc_hash::FxHashMap;

pub type Note = usize;

#[derive(Clone, Default)]
pub struct Mask {
    cells: FxHashMap<Cell, Note>,
}

impl Mask {
    pub fn contains(&self, cell: &Cell) -> bool {
        self.cells.contains_key(cell)
    }

    pub fn check(&mut self, cell: Cell) {
        self.cells.insert(cell, Note::default());
    }

    pub fn uncheck(&mut self, cell: Cell) {
        let _ = self.cells.remove(&cell);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Cell, &Note)> {
        self.cells.iter()
    }
}
