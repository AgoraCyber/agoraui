use std::any::Any;

use uuid::Uuid;

/// Component must implement this trait.
pub trait Component {
    /// Returns the [`id`] of this component
    fn entity(&self) -> &Uuid;
    /// Returns the bound system ID.
    fn system(&self) -> &Uuid;
    /// Convert self to `&dyn Any`
    fn as_any(&self) -> &dyn Any;
    /// Convert self to `&mut dyn Any`
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
