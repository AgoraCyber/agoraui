mod stateful;
pub use stateful::*;

mod stateless;
pub use stateless::*;

mod build_context;
pub use build_context::*;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::View;

/// An instantiation of a ['View'](super::View) at a particular location in the render tree.
pub trait IElement {
    ///  Add this element to the tree in the given slot of the given parent.
    fn mount(&mut self, parent: Option<WeakElement>);
    /// Update element configuration.
    fn update(&mut self, view: View);

    /// Get element parent node.
    fn parent(&self) -> Option<WeakElement>;
    /// Check if this element eq to rhs
    fn eq(&self, rhs: WeakElement) -> bool;
    /// Remove render object from the render tree.
    fn deactivate(&mut self);
}

pub trait ComponentElement: IElement {
    fn deactive_child(&mut self, mut child: Element) {
        if let Some(parent) = child.parent() {
            assert!(self.eq(parent));
        }

        child.deactivate()
    }
}

type RcElement<T> = Rc<RefCell<T>>;
type RcWeakElement<T> = Weak<RefCell<T>>;

#[derive(Debug, Clone)]
pub enum Element {
    /// Empty element do nothing.
    Empty,
    /// Render element for [`Stateful`](crate::Stateful) configuration
    Stateful(RcElement<StatefulElement>),
    /// Render element for [`Stateless`](crate::Stateless) configuration
    Stateless(RcElement<StatelessElement>),
}

impl Element {
    /// Create weak Element version
    pub fn weak(&self) -> WeakElement {
        match self {
            Element::Empty => WeakElement::Empty,
            Element::Stateful(element) => WeakElement::Stateful(Rc::downgrade(element)),
            Element::Stateless(element) => WeakElement::Stateless(Rc::downgrade(element)),
        }
    }

    pub fn view(&self) -> View {
        match self {
            Element::Empty => View::Empty,
            Element::Stateful(element) => View::Stateful(element.borrow().configuration.clone()),
            Element::Stateless(element) => View::Stateless(element.borrow().configuration.clone()),
        }
    }
}

impl IElement for Element {
    /// Implement [`mount`](IElement::mount) function.
    fn mount(&mut self, parent: Option<WeakElement>) {
        match self {
            Element::Empty => {}
            Element::Stateful(element) => element.borrow_mut().mount(parent),
            Element::Stateless(element) => element.borrow_mut().mount(parent),
        }
    }

    fn update(&mut self, view: View) {
        match self {
            Element::Empty => {}
            Element::Stateful(element) => element.borrow_mut().update(view),
            Element::Stateless(element) => element.borrow_mut().update(view),
        }
    }

    fn parent(&self) -> Option<WeakElement> {
        match self {
            Element::Empty => None,
            Element::Stateful(element) => element.borrow().parent(),
            Element::Stateless(element) => element.borrow().parent(),
        }
    }

    fn eq(&self, rhs: WeakElement) -> bool {
        match self {
            Element::Empty => false,
            Element::Stateful(element) => element.borrow().eq(rhs),
            Element::Stateless(element) => element.borrow().eq(rhs),
        }
    }

    fn deactivate(&mut self) {
        match self {
            Element::Empty => {}
            Element::Stateful(element) => element.borrow_mut().deactivate(),
            Element::Stateless(element) => element.borrow_mut().deactivate(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum WeakElement {
    /// Empty element do nothing.
    Empty,

    Stateful(RcWeakElement<StatefulElement>),

    Stateless(RcWeakElement<StatelessElement>),
}

/// Update the given child with the given new configuration.
pub fn update_child<P: ComponentElement>(
    parent: &mut P,
    child: Option<Element>,
    new_configuration: Option<View>,
) -> Option<Element> {
    if new_configuration.is_none() {
        if let Some(child) = child {
            deactive_child(child);
        }

        return None;
    }

    if let Some(mut child) = child {
        let new_configuration = new_configuration.unwrap();

        let _has_same_super_element_type = has_same_super_element_type(&child, &new_configuration);

        let configuration = child.view();

        if configuration == new_configuration {
            // don't update
            return Some(child);
        } else if configuration.to_key_path() == new_configuration.to_key_path() {
            child.update(new_configuration);
        } else {
            parent.deactive_child(child);
        }
    } else {
    }

    None
}

fn has_same_super_element_type(child: &Element, new_configuration: &View) -> bool {
    match child {
        Element::Empty => {
            if let View::Empty = new_configuration {
                return true;
            } else {
                return false;
            }
        }
        _ => return false,
    }
}

/// Move the given element to the list of inactive elements and detach its
/// render object from the render tree.
///   
/// This method stops the given element from being a child of this element by
/// detaching its render object from the render tree and moving the element to
/// the list of inactive elements.
fn deactive_child(_child: Element) {}
