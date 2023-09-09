use std::{cell::RefCell, fmt::Debug, rc::Rc};

use indextree::{Arena, NodeId};

use crate::view::{Configuration, View};

/// Element id in index tree.
pub type ElementId = NodeId;

pub trait BuildContext {}

pub trait Initializer {
    fn initialize(&mut self, id: ElementId);

    fn to_id(&self) -> ElementId;
}

/// Framework call this trait to handle element lifecycle.
pub trait Lifecycle: Initializer + Debug {
    fn to_configuration(&self) -> View;

    fn update(&mut self, configuration: View);

    fn rebuild(&mut self, arena: &mut Arena<Element>);

    /// Mount element into element tree.
    fn mount(&mut self, arena: &mut Arena<Element>, parent: Option<ElementId>) {
        parent.map(|p| p.append(self.to_id(), arena));

        self.rebuild(arena);
    }

    fn update_child(
        &mut self,
        arena: &mut Arena<Element>,
        child: Option<Element>,
        new_configuration: View,
    ) -> Option<ElementId> {
        if let View::Empty = new_configuration {
            if let Some(child) = child {
                self.deactive_child(arena, child.0.borrow().to_id());
            }

            return None;
        }

        let configuration = self.to_configuration();

        if let Some(child) = child {
            if configuration == new_configuration {
                // Skip update child element.
                Some(child.0.borrow().to_id())
            } else if configuration.same_type(&new_configuration)
                && configuration.to_keypath() == new_configuration.to_keypath()
            {
                // Same element type and path with different configuration.
                child.0.borrow_mut().update(new_configuration);
                Some(child.0.borrow().to_id())
            } else {
                self.deactive_child(arena, child.0.borrow().to_id());
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

            element.0.borrow_mut().mount(arena, Some(self.to_id()));
        }

        child_id
    }
}

/// Element wrapper
#[derive(Debug, Clone)]
pub struct Element(pub Rc<RefCell<dyn Lifecycle>>);

impl<T: Lifecycle + 'static> From<T> for Element {
    fn from(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }
}

#[derive(Debug)]
pub struct ElementNode<T: ?Sized, C> {
    pub id: Option<ElementId>,
    pub config: Configuration<T>,
    pub content: C,
}

impl<T: ?Sized, C> Initializer for ElementNode<T, C> {
    fn initialize(&mut self, id: ElementId) {
        self.id = Some(id);
    }

    fn to_id(&self) -> ElementId {
        self.id.expect("Call initialize first")
    }
}
