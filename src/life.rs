use super::*;

use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Clone, Default)]
pub struct Life {
    cells: FxHashSet<Cell>,
}

impl Life {
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn contains(&self, cell: &Cell) -> bool {
        self.cells.contains(cell)
    }

    pub fn populate(&mut self, cell: Cell) {
        self.cells.insert(cell);
    }

    pub fn unpopulate(&mut self, cell: &Cell) {
        let _ = self.cells.remove(cell);
    }

    pub fn tick(&mut self) {
        let mut adjacent_life = FxHashMap::default();

        for cell in &self.cells {
            let _ = adjacent_life.entry(*cell).or_insert(0);

            for neighbor in Cell::neighbors(*cell) {
                let amount = adjacent_life.entry(neighbor).or_insert(0);

                *amount += 1;
            }
        }

        for (cell, amount) in adjacent_life.iter() {
            match amount {
                2 => {}
                3 => {
                    let _ = self.cells.insert(*cell);
                }
                _ => {
                    let _ = self.cells.remove(cell);
                }
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter()
    }
}

impl std::iter::FromIterator<Cell> for Life {
    fn from_iter<I: IntoIterator<Item = Cell>>(iter: I) -> Self {
        Life {
            cells: iter.into_iter().collect(),
        }
    }
}

impl std::fmt::Debug for Life {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Life")
            .field("cells", &self.cells.len())
            .finish()
    }
}
