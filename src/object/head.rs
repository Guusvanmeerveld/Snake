use std::{cell::RefCell, rc::Rc};

use ggez::graphics::{self, Color};

use crate::{direction::Direction, location::Location};

use super::{segment::Segment, Object};

pub struct Head {
    location: Location,
    updated_direction: bool,
    direction: Direction,
    body: Option<Rc<RefCell<Segment>>>,
}

impl Head {
    pub fn new(location: Location) -> Self {
        Self {
            updated_direction: false,
            location,
            direction: Direction::West,
            body: None,
        }
    }

    /// Returns false if the snake could not move forward
    pub fn move_forward(&mut self, width: isize, height: isize) -> bool {
        self.updated_direction = false;

        let new_location = self.direction.step(self.location);

        println!("{}", new_location);

        if !new_location.is_inside((0, width), (0, height)) {
            return false;
        }

        if self.collided(&new_location) {
            return false;
        }

        self.location = new_location;

        true
    }

    pub fn collided(&self, location: &Location) -> bool {
        if &self.location == location {
            true
        } else if let Some(next) = self.body.as_ref() {
            next.borrow().collided(location)
        } else {
            false
        }
    }

    pub fn add_body(&mut self) {
        let old_segment = self.body();

        let segment = Segment::new(self.location(), self.direction(), old_segment);

        self.body = Some(Rc::new(RefCell::new(segment)));
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if !direction.is_inverse_of(&self.direction) && !self.updated_direction {
            self.updated_direction = true;

            self.direction = direction;
        }
    }

    pub fn body(&self) -> Option<Rc<RefCell<Segment>>> {
        self.body.as_ref().map(|body| body.clone())
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) {
        if let Some(body) = self.body.as_ref() {
            body.borrow().draw(canvas)
        }

        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.location.into())
                .color(Color::BLUE),
        );
    }
}

impl Object for Head {
    fn location(&self) -> Location {
        self.location
    }

    fn set_location(&mut self, location: Location) {
        self.location = location;
    }
}
