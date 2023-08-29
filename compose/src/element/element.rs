use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::View;

use super::*;

/// An instantiation of a ['View'](super::View) at a particular location in the render tree.
pub trait IElement {
    ///  Add this element to the tree in the given slot of the given parent.
    fn mount(&mut self, parent: Option<WeakElement>);
}

type RcElement<T> = Rc<RefCell<T>>;
type RcWeakElement<T> = Weak<RefCell<T>>;

#[derive(Clone)]
pub enum Element {
    /// Empty element do nothing.
    Empty,
    /// Element for [`Composite`](super::ICompositeView) view
    Composite(RcElement<CompositeElement>),
}

impl Element {
    /// Create weak Element version
    pub fn weak(&self) -> WeakElement {
        match self {
            Element::Empty => WeakElement::Empty,
            Element::Composite(element) => WeakElement::Composite(Rc::downgrade(element)),
        }
    }

    pub fn view(&self) -> View {
        match self {
            Element::Empty => View::Empty,
            Element::Composite(element) => View::Composite(element.borrow().configuration.clone()),
        }
    }
}

impl IElement for Element {
    /// Implement [`mount`](IElement::mount) function.
    fn mount(&mut self, parent: Option<WeakElement>) {
        match self {
            Element::Empty => {}
            Element::Composite(element) => element.borrow_mut().mount(parent),
        }
    }
}

#[derive(Clone)]
pub enum WeakElement {
    /// Empty element do nothing.
    Empty,
    /// Element for [`Composite`](super::ICompositeView) view
    Composite(RcWeakElement<CompositeElement>),
}

/// Update the given child with the given new configuration.
pub fn update_child(child: Option<Element>, new_configuration: Option<View>) -> Option<Element> {
    if new_configuration.is_none() {
        if let Some(child) = child {
            deactive_child(child);
        }

        return None;
    }

    if let Some(child) = child {
        let new_configuration = new_configuration.unwrap();

        let _has_same_super_element_type = has_same_super_element_type(&child, &new_configuration);

        let configuration = child.view();

        if configuration == new_configuration {
            // don't update
            return Some(child);
        } else if configuration.to_key() == new_configuration.to_key() {
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
        Element::Composite(_) => {
            if let View::Composite(_) = new_configuration {
                return true;
            } else {
                return false;
            }
        }
    }
}

/// Move the given element to the list of inactive elements and detach its
/// render object from the render tree.
///   
/// This method stops the given element from being a child of this element by
/// detaching its render object from the render tree and moving the element to
/// the list of inactive elements.
fn deactive_child(_child: Element) {}
