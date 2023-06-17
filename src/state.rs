use crate::{
    life::Life,
    mask::{Mask, Note},
    Cell, TickError,
};

use rustc_hash::FxHashSet;
use std::future::Future;

#[derive(Default)]
pub struct State {
    life: Life,
    mask: Mask,
    births: FxHashSet<Cell>,
    is_ticking: bool,
}

impl State {
    pub fn with_life(life: Life) -> Self {
        Self {
            life,
            ..Self::default()
        }
    }

    pub fn cell_count(&self) -> usize {
        self.life.len() + self.births.len()
    }

    pub fn contains(&self, cell: &Cell) -> bool {
        self.life.contains(cell) || self.births.contains(cell)
    }

    pub fn mask_contains(&self, cell: &Cell) -> bool {
        self.mask.contains(cell)
    }

    pub fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.life.iter().chain(self.births.iter())
    }

    pub fn mask(&self) -> impl Iterator<Item = (&Cell, &Note)> {
        self.mask.iter()
    }

    pub fn check(&mut self, cell: Cell) {
        self.mask.check(cell);
    }

    pub fn uncheck(&mut self, cell: Cell) {
        self.mask.uncheck(cell);
    }

    pub fn populate(&mut self, cell: Cell) {
        if self.is_ticking {
            self.births.insert(cell);
        } else {
            self.life.populate(cell);
        }
    }

    pub fn unpopulate(&mut self, cell: &Cell) {
        if self.is_ticking {
            let _ = self.births.remove(cell);
        } else {
            self.life.unpopulate(cell);
        }
    }

    pub fn update(&mut self, mut life: Life) {
        self.births.drain().for_each(|cell| life.populate(cell));

        self.life = life;
        self.is_ticking = false;
    }

    pub fn tick(&mut self, amount: usize) -> Option<impl Future<Output = Result<Life, TickError>>> {
        if self.is_ticking {
            return None;
        }

        self.is_ticking = true;

        let mut life = self.life.clone();

        Some(async move {
            tokio::task::spawn_blocking(move || {
                for _ in 0..amount {
                    life.tick();
                }

                life
            })
            .await
            .map_err(|_| TickError::JoinFailed)
        })
    }
}
