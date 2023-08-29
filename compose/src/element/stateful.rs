use std::{cell::RefCell, rc::Rc};

use crate::{Element, Stateful, View};

#[derive(Debug, Clone)]
pub struct StatefulElement {
    pub(crate) configuration: Stateful,
}

impl From<View> for StatefulElement {
    fn from(value: View) -> Self {
        match value {
            View::Stateful(configuration) => StatefulElement { configuration },
            _ => panic!("Convert to StatefulElement failed: invalid view type"),
        }
    }
}

impl From<StatefulElement> for Element {
    fn from(value: StatefulElement) -> Self {
        Element::Stateful(Rc::new(RefCell::new(value)))
    }
}
