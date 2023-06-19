use crate::{life::Life, Cell};

use rustc_hash::FxHashSet;
use std::future::Future;

#[derive(Default)]
pub struct State {
    life: Life,
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

    pub fn contains(&self, cell: &Cell) -> bool {
        self.life.contains(cell) || self.births.contains(cell)
    }

    pub fn randomize(&mut self) {
        self.life.randomize()
    }

    pub fn clear(&mut self) {
        self.life.clear();
    }

    pub fn save(&mut self) {
        self.life.save_state();
    }

    pub fn reset(&mut self) {
        self.life.clear();
        self.life.reset();
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
        // self.births.drain().for_each(|cell| life.populate(cell));

        self.life = life;
        self.is_ticking = false;
    }

    pub fn tick(&mut self, amount: usize) -> Option<impl Future<Output = Life>> {
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
            .unwrap()
        })
    }
}
