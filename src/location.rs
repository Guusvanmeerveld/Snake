use std::fmt::Display;

use ggez::graphics::{self, Rect};
use rand::{rngs::StdRng, Rng};

use crate::constants::SCALE;

#[derive(Debug, Clone, Copy)]
pub struct Location {
    x: isize,
    y: isize,
}

impl Location {
    pub fn random(
        rng: &mut StdRng,
        min_x: isize,
        min_y: isize,
        max_x: isize,
        max_y: isize,
    ) -> Self {
        let x = rng.gen_range(min_x..max_x);
        let y = rng.gen_range(min_y..max_y);

        Self::new(x, y)
    }

    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> isize {
        self.x
    }

    pub fn y(&self) -> isize {
        self.y
    }

    pub fn is_inside(&self, x_range: (isize, isize), y_range: (isize, isize)) -> bool {
        x_range.0 <= self.x && self.x < x_range.1 && y_range.0 <= self.y && self.y < y_range.1
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)?;

        Ok(())
    }
}

impl Default for Location {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl From<Location> for graphics::Rect {
    fn from(location: Location) -> Self {
        Rect::new(
            (location.x() * SCALE) as f32,
            (location.y() * SCALE) as f32,
            SCALE as f32,
            SCALE as f32,
        )
    }
}
