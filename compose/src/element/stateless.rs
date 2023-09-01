use std::{cell::RefCell, rc::Rc};

use super::BuildContext;
use crate::{ComponentElement, Element, IElement, Stateless, View, WeakElement};

#[derive(Debug, Clone)]
pub struct StatelessElement {
    parent: Option<WeakElement>,
    pub(crate) configuration: Stateless,
    child: Option<Element>,
}

impl From<View> for StatelessElement {
    fn from(value: View) -> Self {
        match value {
            View::Stateless(configuration) => StatelessElement {
                configuration,
                parent: None,
                child: None,
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
    fn mount(self_: &mut Rc<RefCell<Self>>, parent: Option<WeakElement>) {
        ComponentElement::compounent_mount(self_, parent);
    }

    fn update(&mut self, view: View) {
        if let View::Stateless(configuration) = view {
            self.configuration = configuration;
        } else {
            panic!("Convert to StatefulElement failed: invalid view type")
        }
    }

    fn set_parent(&mut self, parent: Option<WeakElement>) {
        self.parent = parent;
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

    fn downgrade(self_: &mut Rc<RefCell<Self>>) -> WeakElement {
        WeakElement::Stateless(Rc::downgrade(self_))
    }
}

impl ComponentElement for StatelessElement {
    fn build(&mut self) -> View {
        let config = self.configuration.clone();
        let view = config.configration.borrow().framework_build(self);

        view
    }

    fn child(&mut self) -> &mut Option<Element> {
        &mut self.child
    }

    fn set_child(&mut self, child: Option<Element>) {
        self.child = child;
    }
}
