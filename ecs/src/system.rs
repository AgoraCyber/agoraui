use uuid::Uuid;

use crate::world::World;

pub trait System {
    /// Returns system's id.
    fn id(&self) -> &Uuid;
    /// System update method.
    fn update(&mut self, world: &mut World);
}
