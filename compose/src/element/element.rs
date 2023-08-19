use std::{cell::RefCell, rc::Rc};

use super::*;

/// An instantiation of a ['View'](super::View) at a particular location in the render tree.
pub trait IElement {
    ///  Add this element to the tree in the given slot of the given parent.
    fn mount(&mut self, parent: Option<Element>);
}

type RcElement<T> = Rc<RefCell<T>>;

#[derive(Clone)]
pub enum Element {
    /// Empty element do nothing.
    Empty,
    /// Element for [`Composite`](super::ICompositeView) view
    Composite(RcElement<CompositeElement>),
}

impl IElement for Element {
    /// Implement [`mount`](IElement::mount) function.
    fn mount(&mut self, parent: Option<Element>) {
        match self {
            Element::Empty => {}
            Element::Composite(element) => element.borrow_mut().mount(parent),
        }
    }
}
