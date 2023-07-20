use ggez::graphics::{self, Canvas, Color};

use crate::location::Location;

use super::Object;

pub struct Food {
    location: Location,
}

impl Food {
    pub fn new(location: Location) -> Self {
        Self { location }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.location.into())
                .color(Color::RED),
        );
    }
}

impl Object for Food {
    fn location(&self) -> Location {
        self.location
    }

    fn set_location(&mut self, location: Location) {
        self.location = location;
    }
}
