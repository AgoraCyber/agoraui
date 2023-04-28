use uuid::Uuid;

use crate::world::World;

pub trait System {
    /// System update method.
    fn update(&mut self, world: &mut World);
}

pub trait SystemId {
    fn id() -> &'static Uuid;
}
