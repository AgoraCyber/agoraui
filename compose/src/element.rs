mod wrapper;
use std::{cell::RefCell, rc::Rc};

pub use wrapper::*;

mod context;
pub use context::*;

mod composite;
pub use composite::*;

mod stateful;
pub use stateful::*;

mod stateless;
pub use stateless::*;

mod render;
pub use render::*;

use indextree::{Arena, NodeId};

use crate::view::{
    Configuration, RenderObjectConfiguration, StatefulConfiguration, StatelessConfiguration, View,
};

/// Element reference id
pub type ElementId = NodeId;

pub trait ToConfiguration {
    fn to_configuration(&self) -> View;

    fn update_configuration(&mut self, view: View);
}

pub trait GetChild {
    fn child(&self) -> Option<ElementId>;
}

pub trait Mountable: ElementProvider {
    fn rebuild(&mut self, _arena: &mut Arena<Element>);

    fn mount(&mut self, arena: &mut Arena<Element>, parent: Option<ElementId>) {
        parent.map(|p| p.append(self.to_id(), arena));

        self.rebuild(arena);

        self.set_mount_flag(true);
    }
}

pub trait ElementProvider {
    fn mounted(&self) -> bool;

    fn set_mount_flag(&mut self, flag: bool);

    fn to_id(&self) -> ElementId;
}

#[derive(Debug, Clone)]
pub enum Element {
    Stateful(Rc<RefCell<StatefulElement>>),
    Stateless(Rc<RefCell<StatelessElement>>),
    Render(Rc<RefCell<RenderElement>>),
}

impl Element {
    pub fn mount(&self, arena: &mut Arena<Element>, parent: Option<ElementId>) {
        match self {
            Element::Stateful(e) => e.borrow_mut().mount(arena, parent),
            Element::Stateless(e) => e.borrow_mut().mount(arena, parent),
            Element::Render(e) => e.borrow_mut().mount(arena, parent),
        }
    }

    /// Convert self to [`ElementId`]
    pub fn to_id(&self) -> ElementId {
        match self {
            Element::Stateful(e) => e.borrow_mut().to_id(),
            Element::Stateless(e) => e.borrow_mut().to_id(),
            Element::Render(e) => e.borrow_mut().to_id(),
        }
    }
    /// Convert element to [`view configuration`](View)
    pub fn to_configuration(&self) -> View {
        match self {
            Element::Stateful(e) => e.borrow().to_configuration(),
            Element::Stateless(e) => e.borrow().to_configuration(),
            Element::Render(e) => e.borrow().to_configuration(),
        }
    }

    /// Return mounted flag.
    pub fn mounted(&self) -> bool {
        match self {
            Element::Stateful(e) => e.borrow().mounted(),
            Element::Stateless(e) => e.borrow().mounted(),
            Element::Render(e) => e.borrow().mounted(),
        }
    }

    /// Update element configuration
    pub fn update_configuration(&self, view: View) {
        match self {
            Element::Stateful(e) => e.borrow_mut().update_configuration(view),
            Element::Stateless(e) => e.borrow_mut().update_configuration(view),
            Element::Render(e) => e.borrow_mut().update_configuration(view),
        }
    }

    /// Get child element id.
    pub fn child_id(&self) -> Option<ElementId> {
        match self {
            Element::Stateful(e) => e.borrow_mut().child(),
            Element::Stateless(e) => e.borrow_mut().child(),
            Element::Render(e) => e.borrow_mut().child(),
        }
    }

    /// Get child element.
    pub fn child<'a>(&self, arena: &'a Arena<Element>) -> Option<&'a Element> {
        let id = self.child_id();

        id.map(|id| arena.get(id).unwrap().get())
    }

    /// Get mutable child element.
    pub fn child_mut<'a>(&self, arena: &'a mut Arena<Element>) -> Option<&'a mut Element> {
        let id = self.child_id();

        id.map(|id| arena.get_mut(id).unwrap().get_mut())
    }
}
