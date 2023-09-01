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
    fn mount(self_: &mut Rc<RefCell<Self>>, parent: Option<WeakElement>);
    /// Update element configuration.
    fn update(&mut self, view: View);
    /// Set render element parent to new one.
    fn set_parent(&mut self, parent: Option<WeakElement>);

    /// Get element parent node.
    fn parent(&self) -> Option<WeakElement>;
    /// Check if this element eq to rhs
    fn eq(&self, rhs: WeakElement) -> bool;
    /// Remove render object from the render tree.
    fn deactivate(&mut self);

    fn downgrade(self_: &mut Rc<RefCell<Self>>) -> WeakElement;
}
/// The element that composite one or more child elements
pub trait ComponentElement: IElement + BuildContext {
    fn compounent_mount(self_: &mut Rc<RefCell<Self>>, parent: Option<WeakElement>) {
        self_.borrow_mut().set_parent(parent);

        let new_configuration = self_.borrow_mut().build();

        let child = self_.borrow_mut().child().take();

        let child = ComponentElement::update_child(self_, child, Some(new_configuration));

        self_.borrow_mut().set_child(child);
    }

    /// preformance rebuild this component element.
    fn build(&mut self) -> View;

    /// take child element
    fn child(&mut self) -> &mut Option<Element>;

    /// Store child element
    fn set_child(&mut self, child: Option<Element>);

    /// Move the given element to the list of inactive elements and detach its
    /// render object from the render tree.
    ///   
    /// This method stops the given element from being a child of this element by
    /// detaching its render object from the render tree and moving the element to
    /// the list of inactive elements.

    fn deactive_child(&mut self, mut child: Element) {
        if let Some(parent) = child.parent() {
            assert!(self.eq(parent));
        }

        child.deactivate()
    }

    /// Create an element for the given configuration and add it as a child of this
    /// element in the given slot.
    fn inflate_view(self_: &mut Rc<RefCell<Self>>, configuration: View) -> Option<Element> {
        if let View::Empty = configuration {
            return None;
        }
        // let child = configuration
        let mut child = configuration.to_element();

        child.mount(Some(IElement::downgrade(self_)));

        return Some(child);
    }

    /// Update the given child with the given new configuration.
    fn update_child(
        self_: &mut Rc<RefCell<Self>>,
        child: Option<Element>,
        new_configuration: Option<View>,
    ) -> Option<Element> {
        if new_configuration.is_none() {
            if let Some(child) = child {
                self_.borrow_mut().deactive_child(child);
            }

            return None;
        }
        let new_configuration = new_configuration.unwrap();

        if let Some(mut child) = child {
            let _has_same_super_element_type =
                has_same_super_element_type(&child, &new_configuration);

            let configuration = child.view();

            if configuration == new_configuration {
                // don't update
                return Some(child);
            } else if configuration.to_key_path() == new_configuration.to_key_path() {
                child.update(new_configuration);
                return Some(child);
            } else {
                self_.borrow_mut().deactive_child(child);
            }
        }

        return Self::inflate_view(self_, new_configuration);
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

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Stateful(l0), Self::Stateful(r0)) => l0.as_ptr() == r0.as_ptr(),
            (Self::Stateless(l0), Self::Stateless(r0)) => l0.as_ptr() == r0.as_ptr(),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
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

    /// Implement [`mount`](IElement::mount) function.
    pub fn mount(&mut self, parent: Option<WeakElement>) {
        match self {
            Element::Empty => {}
            Element::Stateful(element) => StatefulElement::mount(element, parent),
            Element::Stateless(element) => StatelessElement::mount(element, parent),
        }
    }

    pub fn update(&mut self, view: View) {
        match self {
            Element::Empty => {}
            Element::Stateful(element) => element.borrow_mut().update(view),
            Element::Stateless(element) => element.borrow_mut().update(view),
        }
    }

    pub fn parent(&self) -> Option<WeakElement> {
        match self {
            Element::Empty => None,
            Element::Stateful(element) => element.borrow().parent(),
            Element::Stateless(element) => element.borrow().parent(),
        }
    }

    pub fn eq(&self, rhs: WeakElement) -> bool {
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

    /// Try get element child node, the leaf element always returns [`None`]
    pub fn child(&self) -> Option<Element> {
        match self {
            Element::Empty => None,
            Element::Stateful(element) => element.borrow_mut().child().clone(),
            Element::Stateless(element) => element.borrow_mut().child().clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum WeakElement {
    /// Empty element do nothing.
    Empty,
    /// Render element for [`Stateful`](crate::Stateful) configuration
    Stateful(RcWeakElement<StatefulElement>),
    /// Render element for [`Stateless`](crate::Stateless) configuration
    Stateless(RcWeakElement<StatelessElement>),
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
