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

pub trait UpdateChild: ToConfiguration + ElementProvider {
    fn update_child(
        &mut self,
        arena: &mut Arena<Element>,
        child: Option<Element>,
        new_configuration: View,
    ) -> Option<ElementId> {
        if let View::Empty = new_configuration {
            if let Some(child) = child {
                self.deactive_child(arena, child.to_id());
            }

            return None;
        }

        let configuration = self.to_configuration();

        if let Some(child) = child {
            if configuration == new_configuration {
                // Skip update child element.
                Some(child.to_id())
            } else if configuration.same_type(&new_configuration)
                && configuration.to_keypath() == new_configuration.to_keypath()
            {
                // Same element type and path with different configuration.
                child.update_configuration(new_configuration);
                Some(child.to_id())
            } else {
                self.deactive_child(arena, child.to_id());
                self.inflate_view(arena, new_configuration)
            }
        } else {
            self.inflate_view(arena, new_configuration)
        }
    }

    fn deactive_child(&mut self, arena: &mut Arena<Element>, id: ElementId) {
        id.remove(arena);
    }

    fn inflate_view(&mut self, arena: &mut Arena<Element>, configuration: View) -> Option<NodeId> {
        let child_id = configuration.into_element(arena);

        if let Some(child_id) = child_id {
            let element = arena.get(child_id).unwrap().get().clone();

            element.mount(arena, Some(self.to_id()));
        }

        child_id
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
