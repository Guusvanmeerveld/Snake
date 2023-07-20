use std::{cell::RefCell, rc::Rc};

use ggez::graphics::{self, Color};

use crate::{direction::Direction, location::Location};

use super::Object;

pub struct Segment {
    location: Location,
    direction: Direction,
    next_segment: Option<Rc<RefCell<Segment>>>,
}

impl Segment {
    pub fn follow(&mut self, direction: Direction) {
        self.set_location(self.direction.step(self.location));

        if let Some(next) = self.next_segment.as_ref() {
            next.borrow_mut().follow(self.direction);
        }

        self.direction = direction;
    }

    pub fn collided(&self, location: &Location) -> bool {
        if &self.location == location {
            true
        } else if let Some(next) = self.next_segment.as_ref() {
            next.borrow().collided(location)
        } else {
            false
        }
    }

    pub fn new(
        location: Location,
        direction: Direction,
        next_segment: Option<Rc<RefCell<Segment>>>,
    ) -> Self {
        Self {
            direction,
            location,
            next_segment,
        }
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) {
        if let Some(next) = self.next_segment.as_ref() {
            next.borrow().draw(canvas);
        }

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.location.into())
                .color(Color::GREEN),
        );
    }
}

impl Object for Segment {
    fn location(&self) -> Location {
        self.location
    }

    fn set_location(&mut self, location: Location) {
        self.location = location;
    }
}
