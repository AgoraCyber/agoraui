use std::{cell::RefCell, rc::Rc};

use super::*;
use crate::view::View;

///  Element to handle [`Composite`](super::ICompositeView) view
#[allow(dead_code)]
pub struct CompositeElement {
    view: View,
}

impl CompositeElement {
    pub fn new(view: View) -> Self {
        return Self { view };
    }
}

impl IBuildContext for CompositeElement {
    fn set_state(&self) {
        todo!()
    }
}

impl IElement for CompositeElement {
    fn mount(&mut self, _parent: Option<Element>) {}
}

impl From<CompositeElement> for Element {
    fn from(value: CompositeElement) -> Self {
        Element::Composite(Rc::new(RefCell::new(value)))
    }
}
