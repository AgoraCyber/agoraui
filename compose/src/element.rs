mod wrapper;
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

pub trait ElementService {
    fn to_id(&self) -> ElementId;
    fn rebuild(&mut self) {}

    fn mount(&mut self, arena: &mut Arena<Element>, parent: Option<ElementId>) {
        parent.map(|p| p.append(self.to_id(), arena));

        self.rebuild();
    }
}

#[derive(Debug)]
pub enum Element {
    Stateful(StatefulElement),
    Stateless(StatelessElement),
    Render(RenderElement),
}

impl ElementService for Element {
    fn mount(&mut self, arena: &mut Arena<Element>, parent: Option<ElementId>) {
        match self {
            Element::Stateful(e) => e.mount(arena, parent),
            Element::Stateless(e) => e.mount(arena, parent),
            Element::Render(e) => e.mount(arena, parent),
        }
    }
    fn to_id(&self) -> ElementId {
        match self {
            Element::Stateful(e) => e.to_id(),
            Element::Stateless(e) => e.to_id(),
            Element::Render(e) => e.to_id(),
        }
    }
}
