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

pub trait ElementProvider {
    fn mounted(&self) -> bool;

    fn set_mount_flag(&mut self, flag: bool);

    fn to_id(&self) -> ElementId;

    fn rebuild(&mut self) {}

    fn mount(&mut self, arena: &mut Arena<Element>, parent: Option<ElementId>) {
        parent.map(|p| p.append(self.to_id(), arena));

        self.rebuild();

        self.set_mount_flag(true);
    }
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
            Element::Stateful(e) => View::Stateful(e.borrow().config.clone()),
            Element::Stateless(e) => View::Stateless(e.borrow().config.clone()),
            Element::Render(e) => View::RenderObject(e.borrow().config.clone()),
        }
    }

    pub fn mounted(&self) -> bool {
        match self {
            Element::Stateful(e) => e.borrow().mounted(),
            Element::Stateless(e) => e.borrow().mounted(),
            Element::Render(e) => e.borrow().mounted(),
        }
    }
}
