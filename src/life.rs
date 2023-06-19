use super::*;

use itertools::Itertools;
use rand::random;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Clone, Default)]
pub struct Life {
    seed: FxHashSet<Cell>,
    cells: FxHashSet<Cell>,
}

impl Life {
    pub fn contains(&self, cell: &Cell) -> bool {
        self.cells.contains(cell)
    }

    pub fn populate(&mut self, cell: Cell) {
        self.cells.insert(cell);
    }

    pub fn unpopulate(&mut self, cell: &Cell) {
        self.cells.remove(cell);
    }

    pub fn clear(&mut self) {
        self.cells = FxHashSet::default();
    }

    pub fn reset(&mut self) {
        self.cells = self.seed.clone();
    }

    pub fn save_state(&mut self) {
        self.seed = self.cells.clone();
    }

    pub fn randomize(&mut self) {
        self.cells.clear();
        for (i, j) in (-32..=32).cartesian_product(-32..=32) {
            if random::<f32>() > 0.5 {
                self.populate(Cell { i, j })
            }
        }
        self.seed = self.cells.clone();
    }

    pub fn tick(&mut self) {
        let mut adjacent_life = FxHashMap::default();

        for cell in &self.cells {
            adjacent_life.entry(*cell).or_insert(0);

            for neighbor in Cell::neighbors(*cell) {
                let amount = adjacent_life.entry(neighbor).or_insert(0);

                *amount += 1;
            }
        }

        for (cell, amount) in adjacent_life.iter() {
            match amount {
                2 => {}
                3 => {
                    self.cells.insert(*cell);
                }
                _ => {
                    self.cells.remove(cell);
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
        let cells: FxHashSet<Cell> = iter.into_iter().collect();
        Life {
            seed: cells.clone(),
            cells,
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
