use std::{cell::RefCell, rc::Rc};

use crate::{Element, Stateful, View, WeakElement};

use super::BuildContext;

#[derive(Debug, Clone)]
pub struct StatefulElement {
    pub(crate) parent: Option<WeakElement>,
    pub(crate) configuration: Stateful,
}

impl From<View> for StatefulElement {
    fn from(value: View) -> Self {
        match value {
            View::Stateful(configuration) => StatefulElement {
                configuration,
                parent: None,
            },
            _ => panic!("Convert to StatefulElement failed: invalid view type"),
        }
    }
}

impl From<StatefulElement> for Element {
    fn from(value: StatefulElement) -> Self {
        Element::Stateful(Rc::new(RefCell::new(value)))
    }
}

impl BuildContext for StatefulElement {}
