use std::{cell::RefCell, rc::Rc};

use indextree::Arena;

use crate::{element::Element, view::RenderObject};

#[derive(Debug, Default, Clone)]
pub struct FrameworkContext {
    pub element_tree: Rc<RefCell<Arena<Element>>>,
    pub render_tree: Rc<RefCell<Arena<RenderObject>>>,
}

impl
    From<(
        Rc<RefCell<Arena<Element>>>,
        Rc<RefCell<Arena<RenderObject>>>,
    )> for FrameworkContext
{
    fn from(
        value: (
            Rc<RefCell<Arena<Element>>>,
            Rc<RefCell<Arena<RenderObject>>>,
        ),
    ) -> Self {
        FrameworkContext {
            element_tree: value.0,
            render_tree: value.1,
        }
    }
}
