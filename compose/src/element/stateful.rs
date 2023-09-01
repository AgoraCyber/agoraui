use std::{cell::RefCell, rc::Rc};

use crate::{ComponentElement, Element, IElement, State, Stateful, View, WeakElement};

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
    fn mount(self_: &mut Rc<RefCell<Self>>, parent: Option<WeakElement>) {
        ComponentElement::compounent_mount(self_, parent);
    }

    fn set_parent(&mut self, parent: Option<WeakElement>) {
        self.parent = parent;
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

    fn downgrade(self_: &mut Rc<RefCell<Self>>) -> WeakElement {
        WeakElement::Stateful(Rc::downgrade(self_))
    }
}

impl ComponentElement for StatefulElement {
    /// preformance rebuild this component element.
    fn build(&mut self) -> View {
        let state = self.state.clone();

        let view = state.borrow_mut().framework_build(self);

        view
    }

    /// take child element
    fn child(&mut self) -> &mut Option<Element> {
        &mut self.child
    }

    /// Store child element
    fn set_child(&mut self, child: Option<Element>) {
        self.child = child;
    }
}
