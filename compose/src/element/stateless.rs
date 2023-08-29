use std::{cell::RefCell, rc::Rc};

use crate::{Element, Stateless, View};

#[derive(Debug, Clone)]
pub struct StatelessElement {
    pub(crate) configuration: Stateless,
}

impl From<View> for StatelessElement {
    fn from(value: View) -> Self {
        match value {
            View::Stateless(configuration) => StatelessElement { configuration },
            _ => panic!("Convert to StatelessElement failed: invalid view type"),
        }
    }
}

impl From<StatelessElement> for Element {
    fn from(value: StatelessElement) -> Self {
        Element::Stateless(Rc::new(RefCell::new(value)))
    }
}
