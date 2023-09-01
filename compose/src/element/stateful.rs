use std::{cell::RefCell, rc::Rc};

use crate::{
    update_child, ComponentElement, Element, IElement, State, Stateful, View, WeakElement,
};

use super::BuildContext;

#[derive(Debug, Clone)]
pub struct StatefulElement {
    parent: Option<WeakElement>,
    state: Rc<RefCell<Box<dyn State>>>,
    child: Option<Element>,
    pub(crate) configuration: Stateful,
}

impl From<View> for StatefulElement {
    fn from(value: View) -> Self {
        match value {
            View::Stateful(configuration) => {
                let state = configuration.configration.borrow().framework_create_state();
                StatefulElement {
                    configuration,
                    parent: None,
                    child: None,
                    state: Rc::new(RefCell::new(state)),
                }
            }
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

impl IElement for StatefulElement {
    fn mount(&mut self, parent: Option<WeakElement>) {
        self.parent = parent;

        let state = self.state.clone();

        let built = state.borrow_mut().framework_build(self);

        let child = self.child.take();

        self.child = update_child(self, child, Some(built));
    }

    fn update(&mut self, view: View) {
        if let View::Stateful(configuration) = view {
            self.configuration = configuration;
        } else {
            panic!("Convert to StatefulElement failed: invalid view type")
        }
    }

    fn eq(&self, element: WeakElement) -> bool {
        if let WeakElement::Stateful(element) = element {
            unsafe { self as *const StatefulElement == (*element.as_ptr()).as_ptr() }
        } else {
            false
        }
    }

    fn parent(&self) -> Option<WeakElement> {
        self.parent.clone()
    }

    fn deactivate(&mut self) {}
}

impl ComponentElement for StatefulElement {}
