use super::*;

/// A handle to the location of a view in the view tree.
pub trait IBuildContext {
    fn set_state(&self);
}

/// Type erased [IBuildContext] implementation.
#[derive(Clone)]
pub struct BuildContext {
    element: Element,
}

impl From<Element> for BuildContext {
    fn from(value: Element) -> Self {
        BuildContext { element: value }
    }
}

impl BuildContext {
    pub fn set_state(&self) {
        match &self.element {
            Element::Empty => {}
            Element::Composite(e) => e.borrow().set_state(),
        }
    }
}
