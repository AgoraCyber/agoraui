use std::{cell::RefCell, fmt::Debug, rc::Rc};

use indextree::NodeId;

pub trait RenderObjectLifecycle: Debug {}

pub type RenderObjectId = NodeId;

#[derive(Debug)]
pub struct RenderObjectNode {
    pub id: Option<RenderObjectId>,
    pub lifecycle: Box<dyn RenderObjectLifecycle>,
}

/// Element wrapper
#[derive(Debug, Clone)]
pub struct RenderObject(pub Rc<RefCell<RenderObjectNode>>);

impl<T: RenderObjectLifecycle + 'static> From<T> for RenderObject {
    fn from(value: T) -> Self {
        Self(Rc::new(RefCell::new(RenderObjectNode {
            id: None,
            lifecycle: Box::new(value),
        })))
    }
}

impl RenderObject {
    pub fn initialize(&mut self, id: RenderObjectId) {
        self.0.borrow_mut().id = Some(id);
    }
}
