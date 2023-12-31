use iced::{
    mouse::{self, Button::Left, Event::ButtonPressed},
    widget::canvas::{
        event::{self, Event},
        Cache, Canvas, Cursor, Geometry, Path, Program,
    },
    {Color, Element, Length, Point, Rectangle, Size, Theme},
};

use super::*;

use itertools::Itertools;
use rand::random;
use rustc_hash::FxHashMap;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Map {
    seed: CellMap,
    loop_point: CellMap,
    cells: CellMap,
    life_cache: Cache,
    randomness: f32,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            seed: CellMap::default(),
            cells: CellMap::default(),
            loop_point: CellMap::default(),
            life_cache: Cache::default(),
            randomness: 0.5,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Populate(Cell),
    Unpopulate(Cell),
    Ticked(CellMap),
}

impl Map {
    pub fn set_loop(&mut self) {
        self.loop_point = self.cells.clone();
    }

    pub fn reset_loop(&mut self) -> CellMap {
        self.loop_point.clone()
    }

    pub fn tick(&self) -> CellMap {
        let mut life = self.cells.clone();
        let mut counts = FxHashMap::default();

        for cell in &life {
            counts.entry(*cell).or_insert(0);

            for neighbor in Cell::neighbors(*cell) {
                let amount = counts.entry(neighbor).or_insert(0);

                *amount += 1;
            }
        }

        for (cell, amount) in counts.iter() {
            match amount {
                2 => {}
                3 => {
                    life.insert(*cell);
                }
                _ => {
                    life.remove(cell);
                }
            }
        }

        life
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Populate(cell) => {
                self.cells.insert(cell);
                self.life_cache.clear();
            }
            Message::Unpopulate(cell) => {
                self.cells.remove(&cell);
                self.life_cache.clear();
            }
            Message::Ticked(life) => {
                self.cells = life;
                self.life_cache.clear();
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fixed(Cell::SIZE as f32 * 24.0))
            .height(Length::Fixed(Cell::SIZE as f32 * 24.0))
            .into()
    }

    pub fn clear(&mut self) {
        self.cells.clear();
        self.life_cache.clear();
    }

    pub fn reset(&mut self) {
        self.cells = self.seed.clone();
        self.life_cache.clear();
    }

    pub fn save(&mut self) {
        self.seed = self.cells.clone();
    }

    pub fn randomize(&mut self) {
        self.cells.clear();
        for (i, j) in (-32..=32).cartesian_product(-32..=32) {
            if random::<f32>() < self.randomness {
                self.cells.insert(Cell { i, j });
            }
        }
        self.seed = self.cells.clone();
        self.life_cache.clear();
    }

    pub fn randomness(&self) -> f32 {
        self.randomness
    }

    pub fn set_randomness(&mut self, value: f32) {
        self.randomness = value;
    }
}

impl Program<Message> for Map {
    type State = bool;

    fn update(
        &self,
        _interaction: &mut bool,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        if let Some(position) = cursor.position_in(&bounds) {
            if let Event::Mouse(ButtonPressed(Left)) = event {
                let cell = Cell::at(position);
                return (
                    event::Status::Captured,
                    if self.cells.contains(&cell) {
                        Some(Message::Unpopulate(cell))
                    } else {
                        Some(Message::Populate(cell))
                    },
                );
            }
        }

        (event::Status::Ignored, None)
    }

    fn draw(
        &self,
        _interaction: &bool,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        vec![self.life_cache.draw(bounds.size(), |frame| {
            let background = Path::rectangle(Point::ORIGIN, frame.size());
            frame.fill(&background, Color::from_rgb8(0x30, 0x30, 0x30));

            frame.with_save(|frame| {
                frame.scale(Cell::SIZE as f32);

                (0..24)
                    .cartesian_product(0..24)
                    .filter(|x| self.cells.contains(&Cell { i: x.1, j: x.0 }))
                    .for_each(|x| {
                        frame.fill_rectangle(
                            Point::new(x.0 as f32, x.1 as f32),
                            Size::UNIT,
                            Color::from_rgb8(0xD7, 0xD0, 0xC7),
                        );
                    })
            });
        })]
    }

    fn mouse_interaction(
        &self,
        _interaction: &bool,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> mouse::Interaction {
        mouse::Interaction::default()
    }
}
