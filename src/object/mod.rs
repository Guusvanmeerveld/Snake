use crate::location::Location;

mod food;
mod head;
mod segment;

pub use food::Food;
pub use head::Head;
pub use segment::Segment;

pub trait Object {
    fn location(&self) -> Location;

    fn set_location(&mut self, location: Location);

    fn same_location<Other: Object>(&self, other: &Other) -> bool {
        self.location() == other.location()
    }
}
