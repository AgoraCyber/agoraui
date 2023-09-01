use std::{cell::RefCell, rc::Rc};

use super::BuildContext;
use crate::{Element, IElement, Stateless, View, WeakElement};

#[derive(Debug, Clone)]
pub struct StatelessElement {
    pub(crate) parent: Option<WeakElement>,
    pub(crate) configuration: Stateless,
}

impl From<View> for StatelessElement {
    fn from(value: View) -> Self {
        match value {
            View::Stateless(configuration) => StatelessElement {
                configuration,
                parent: None,
            },
            _ => panic!("Convert to StatelessElement failed: invalid view type"),
        }
    }
}

impl From<StatelessElement> for Element {
    fn from(value: StatelessElement) -> Self {
        Element::Stateless(Rc::new(RefCell::new(value)))
    }
}

impl BuildContext for StatelessElement {}

impl IElement for StatelessElement {
    fn mount(&mut self, parent: Option<WeakElement>) {
        self.parent = parent;
    }

    fn update(&mut self, view: View) {
        if let View::Stateless(configuration) = view {
            self.configuration = configuration;
        } else {
            panic!("Convert to StatefulElement failed: invalid view type")
        }
    }

    fn parent(&self) -> Option<WeakElement> {
        self.parent.clone()
    }

    fn eq(&self, element: WeakElement) -> bool {
        if let WeakElement::Stateless(element) = element {
            unsafe { self as *const StatelessElement == (*element.as_ptr()).as_ptr() }
        } else {
            false
        }
    }

    fn deactivate(&mut self) {}
}
