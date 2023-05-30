use crossterm::{
    cursor::{Hide, MoveTo},
    queue,
    style::{
        Color::{Black, White},
        Colors, SetColors,
    },
};
use eyre::Result;

use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Area {
    pub origin: Point,
    pub max: Point,
    pub colors: Option<Colors>,
}

impl From<(Point, Point)> for Area {
    fn from(value: (Point, Point)) -> Self {
        Self {
            origin: value.0,
            max: value.1,
            colors: None,
        }
    }
}

impl Area {
    pub fn new(x_zero: usize, y_zero: usize, x_max: usize, y_max: usize) -> Self {
        Self {
            origin: Point::new(x_zero, y_zero),
            max: Point::new(x_max, y_max),
            colors: None,
        }
    }

    pub fn set_colors(&mut self, colors: Colors) {
        self.colors = Some(colors);
    }

    pub fn to_u16(&self) -> Result<((u16, u16), (u16, u16))> {
        Ok((
            (self.origin.x.try_into()?, self.origin.y.try_into()?),
            (self.max.x.try_into()?, self.max.y.try_into()?),
        ))
    }

    pub fn outline_area(&self) -> Result<()> {
        let colors = self.colors.unwrap_or(Colors::new(White, Black));
        let ((x_zero, y_zero), (x_max, y_max)) = self.to_u16()?;

        for y in y_zero..=y_max {
            queue!(
                stdout(),
                Hide,
                MoveTo(x_zero, y),
                SetColors(colors),
                Print("┃"),
                MoveTo(x_max, y),
                SetColors(colors),
                Print("┃")
            )?;
        }

        for x in x_zero..=x_max {
            queue!(
                stdout(),
                Hide,
                MoveTo(x, y_zero),
                SetColors(colors),
                Print("━"),
                MoveTo(x, y_max),
                SetColors(colors),
                Print("━")
            )?;
        }

        for ((x, y), c) in [
            (x_zero, y_zero),
            (x_max, y_zero),
            (x_zero, y_max),
            (x_max, y_max),
        ]
        .iter()
        .zip(['┏', '┓', '┗', '┛'].iter())
        {
            queue!(stdout(), MoveTo(*x, *y), SetColors(colors), Print(c))?;
        }

        Ok(())
    }
}
