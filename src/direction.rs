use crate::location::Location;

use ggez::input::keyboard::KeyCode;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn step(&self, current_location: Location) -> Location {
        match &self {
            Direction::North => {
                let y = current_location.y().saturating_sub(1);

                Location::new(current_location.x(), y)
            }
            Direction::East => {
                let x = current_location.x().saturating_add(1);

                Location::new(x, current_location.y())
            }
            Direction::South => {
                let y = current_location.y().saturating_add(1);

                Location::new(current_location.x(), y)
            }
            Direction::West => {
                let x = current_location.x().saturating_sub(1);

                Location::new(x, current_location.y())
            }
        }
    }

    pub fn is_inverse_of(&self, direction: &Direction) -> bool {
        match self {
            Direction::North => direction == &Direction::South,
            Direction::East => direction == &Direction::West,
            Direction::South => direction == &Direction::North,
            Direction::West => direction == &Direction::East,
        }
    }

    pub fn from_keycode(key: KeyCode) -> Option<Direction> {
        match key {
            KeyCode::Up => Some(Direction::North),
            KeyCode::W => Some(Direction::North),
            KeyCode::Down => Some(Direction::South),
            KeyCode::S => Some(Direction::South),
            KeyCode::Left => Some(Direction::West),
            KeyCode::A => Some(Direction::West),
            KeyCode::Right => Some(Direction::East),
            KeyCode::D => Some(Direction::East),
            _ => None,
        }
    }
}
