use std::{cell::RefCell, rc::Rc};

use super::*;
use crate::{view::View, CompositeView};

///  Element to handle [`Composite`](super::ICompositeView) view
#[allow(dead_code)]
pub struct CompositeElement {
    parent: Option<WeakElement>,
    child: Option<Element>,
    pub(crate) configuration: CompositeView,
}

impl CompositeElement {
    /// Create new [`CompositeElement`] from View.
    pub fn new(configuration: View) -> Self {
        match configuration {
            View::Composite(configuration) => {
                return Self {
                    configuration,
                    parent: None,
                    child: None,
                }
            }
            _ => panic!("only CompositeView accept"),
        }
    }

    /// Rebuild whole element with view configuration.
    fn rebuild(&mut self) {
        let view = self.configuration.build();

        self.child = update_child(self.child.take(), Some(view));
    }
}

impl IBuildContext for CompositeElement {
    fn set_state(&self) {
        todo!()
    }
}

impl IElement for CompositeElement {
    fn mount(&mut self, parent: Option<WeakElement>) {
        self.parent = parent;
        self.rebuild()
    }
}

impl From<CompositeElement> for Element {
    fn from(value: CompositeElement) -> Self {
        Element::Composite(Rc::new(RefCell::new(value)))
    }
}
